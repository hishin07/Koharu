<<<<<<< HEAD
//! Qwen Image 2.1 generation, editing, and inpainting.
//!
//! https://github.com/leejet/stable-diffusion.cpp/blob/2f886889e6e8b78738d6b87f7191f6018557c551/docs/qwen_image_2.1.md

mod model;
mod processor;

use anyhow::{Context, Result, ensure};
use fast_image_resize::{FilterType, ResizeAlg, ResizeOptions, Resizer};
use image::{DynamicImage, RgbImage, RgbaImage};
use koharu_diffusion::{GuidanceParams, ImageGenerationParams, SampleMethod, SampleParams};

use self::{
    model::{Model, ModelPaths},
    processor::QwenImageProcessor,
};

pub use self::processor::{QwenImageInpaintOptions, QwenImageOptions};

crate::model_repository!("leejet/Qwen-Image-2.1-GGUF" @ "cc11433936a06e9765f7c0c0b1f0436cfd2b9856" {
    DIFFUSION_WEIGHTS = "qwen_image_2.1-Q4_K.gguf"
});
// The uncensored checkpoint carries the same architecture, converted with
// stable-diffusion.cpp upstream: only the diffusion weights swap, while the
// Heretic text encoder and the shared VAE stay.
crate::model_repository!("abenzerps/Qwen-Image-2.1-Uncensored-GGUF" @ "6b34e59458d3eb7ba6a6f86a116aed5253dc02c3" {
    UNCENSORED_DIFFUSION_WEIGHTS = "qwen-image-2.1-UC-Q4_K_M.gguf"
});
crate::model_repository!("pottokao/Qwen-Image-2.1-Text-Encoder-Heretic-GGUF" @ "23813717f7f9282b372c23c7207468a7a168fa68" {
    TEXT_ENCODER_WEIGHTS = "qwen3vl_8b_heretic-Q4_K_M.gguf",
    TEXT_ENCODER_VISION_WEIGHTS = "mmproj-qwen3vl_8b_heretic-f16.gguf"
});
crate::model_repository!("Comfy-Org/Qwen-Image-2.1" @ "9a44dbdb47cefd046be9c0a13476192f34c8db8e" {
    VAE_WEIGHTS = "vae/qwen_image_2.1_vae_bf16.safetensors"
});

/// Upstream recommends `--cfg-scale 6.0` for Qwen Image 2.1.
const CFG_SCALE: f32 = 6.0;
=======
//! Qwen-Image 2.1 Turbo generation and editing.
//!
//! Follows diffusers' `QwenImage21Pipeline` as the Turbo checkpoint configures
//! it: flow-matching Euler steps through the 8 sigmas it saves, which set the
//! step count, no guidance, and a 1024-pixel side.
//! https://github.com/huggingface/diffusers/blob/74fb49b5506a6960d55de0f7e6f4e0631213dc63/src/diffusers/pipelines/qwenimage21/pipeline_qwenimage21.py
//! https://huggingface.co/Qwen/Qwen-Image-2.1-Turbo/blob/fc13ba1986fcb905854dc8de2f52eee395f49b5b/model_index.json
//! Editing is generation conditioned on images, which the model reads in the
//! order given. A mask or a painted annotation is one more condition image
//! the prompt names, as Qwen-Image 2.1's model card describes local edits.
//! https://huggingface.co/Qwen/Qwen-Image-2.1/blob/d26bb61231c349cf6b7896fa83353113880e1ba3/README.md

mod model;

use anyhow::{Context, Result, ensure};
use fast_image_resize::{FilterType, ResizeAlg};
use image::{DynamicImage, RgbImage, RgbaImage};
use koharu_diffusion::{Canceller, ImageParams, SampleMethod};
use koharu_runtime::Repository;

use self::model::{Model, ModelPaths};

/// unsloth's GGUF of Qwen-Image 2.1 Turbo, whose text encoder, vision
/// projector, and VAE are 2.1's.
const TRANSFORMER: Repository = Repository::model(
    "unsloth/Qwen-Image-2.1-Turbo-GGUF",
    "bd53e520a64472adaff780da0b2f2d7eb1605e30",
);
/// Qwen-Image 2.1's own VAE, which stable-diffusion.cpp's guide names; the
/// earlier Qwen-Image and Wan 2.2 VAEs don't stand in for it.
const VAE: Repository = Repository::model(
    "Comfy-Org/Qwen-Image-2.1",
    "aacf31a6d48e707f352ecba714cee0680da3479b",
);
/// Qwen's own GGUF of the Qwen3-VL 8B text encoder, with the vision projector
/// that reads condition images.
const TEXT_ENCODER: Repository = Repository::model(
    "Qwen/Qwen3-VL-8B-Instruct-GGUF",
    "f982a07559d4a2f6c8744d840bf6fccab30eea96",
);

/// The VAE's 16× downscale by the transformer's 2×2 patches: sizes are
/// multiples of it.
const MULTIPLE: u32 = 32;
/// diffusers' `VaeImageProcessor.resize`, a PIL Lanczos.
const RESIZE: ResizeAlg = ResizeAlg::Convolution(FilterType::Lanczos3);
/// Turbo's `sample_sigmas`, and the terminal zero diffusers' scheduler
/// appends and sd.cpp takes in the list.
const SIGMAS: [f32; 9] = [
    1.0, 0.978453, 0.95418, 0.926626, 0.89508, 0.845148, 0.704534, 0.414568, 0.0,
];
/// The steps `SIGMAS` make.
const STEPS: usize = SIGMAS.len() - 1;

/// `QwenImage21Pipeline.__call__`'s parameters and defaults, but the steps,
/// which Turbo's sigmas set.
#[derive(Debug, Clone, PartialEq)]
pub struct Options {
    /// Steers away with classifier-free guidance, when `true_cfg_scale` is
    /// above 1.
    pub negative_prompt: Option<String>,
    pub true_cfg_scale: f32,
    /// Unset, `output_resolution`'s side, or the last condition image's
    /// aspect ratio at its area; rounded down to a multiple of 32.
    pub height: Option<u32>,
    pub width: Option<u32>,
    pub num_images_per_prompt: u32,
    /// sd.cpp's seed; -1 picks one at random.
    pub seed: i64,
    /// The side the output and every condition image are sized to the area of.
    pub output_resolution: u32,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            negative_prompt: None,
            true_cfg_scale: 1.0,
            height: None,
            width: None,
            num_images_per_prompt: 1,
            seed: -1,
            output_resolution: 1024,
        }
    }
}
>>>>>>> upstream/main

#[derive(Debug)]
pub struct QwenImage {
    model: Model,
}

impl QwenImage {
<<<<<<< HEAD
    pub async fn load(device: crate::Device) -> Result<Self> {
        let started = std::time::Instant::now();
        let (diffusion_model, text_encoder, text_encoder_vision, vae) = tokio::try_join!(
            DIFFUSION_WEIGHTS.resolve(),
            TEXT_ENCODER_WEIGHTS.resolve(),
            TEXT_ENCODER_VISION_WEIGHTS.resolve(),
            VAE_WEIGHTS.resolve(),
        )
        .context("failed to resolve Qwen Image 2.1 model assets")?;
        let model = Model::new(
            &device,
            ModelPaths {
                diffusion_model,
                text_encoder,
                text_encoder_vision,
                vae,
            },
        )?;
        tracing::info!(
            elapsed = format!("{:.1}s", started.elapsed().as_secs_f32()),
            "loaded Qwen Image 2.1"
        );
        Ok(Self { model })
    }

    /// Generates images from `prompt`. Passing `image` switches to editing mode:
    /// upstream conditions on the reference images instead of generating from
    /// noise alone.
    pub fn inference(
        &self,
        image: &[DynamicImage],
        prompt: &str,
        options: &QwenImageOptions,
=======
    /// What stops a generation under way, from another thread.
    pub fn canceller(&self) -> Canceller {
        self.model.canceller.clone()
    }

    /// Loads the model once for generation and editing, its weights placed by
    /// stable-diffusion.cpp's own fit to the device's free memory.
    pub async fn load(device: &crate::Device) -> Result<Self> {
        let paths = async {
            // 4-bit, as Koharu's other diffusion models load.
            let [transformer] = TRANSFORMER
                .resolve(["qwen-image-2.1-turbo-Q4_K_M.gguf"])
                .await?;
            let [vae] = VAE
                .resolve(["vae/qwen_image_2.1_vae_bf16.safetensors"])
                .await?;
            let [text_encoder, vision] = TEXT_ENCODER
                .resolve([
                    "Qwen3VL-8B-Instruct-Q4_K_M.gguf",
                    "mmproj-Qwen3VL-8B-Instruct-F16.gguf",
                ])
                .await?;
            anyhow::Ok(ModelPaths {
                transformer,
                text_encoder,
                vision,
                vae,
            })
        }
        .await
        .context("failed to resolve Qwen-Image 2.1 Turbo's files")?;
        let model = Model::new(device, &paths)?;
        Ok(Self { model })
    }

    /// Images of `prompt`, conditioned on `images` when there are any. They
    /// carry alpha, which the prompt asks for as the model card describes.
    pub fn generate(
        &self,
        prompt: &str,
        images: &[DynamicImage],
        options: &Options,
>>>>>>> upstream/main
    ) -> Result<Vec<RgbaImage>> {
        ensure!(
            !prompt.contains('\0'),
            "prompt contains an interior NUL byte"
        );
        ensure!(
<<<<<<< HEAD
            options.num_inference_steps > 0,
            "num_inference_steps must be greater than zero"
        );
        ensure!(
            options.num_images_per_prompt > 0,
            "num_images_per_prompt must be greater than zero"
        );

        let mut reference_images = Vec::with_capacity(image.len());
        for image in image {
            QwenImageProcessor::check_image_input(image)?;
            reference_images.push(image.clone().to_rgb8());
        }

        let height = options
            .height
            .or_else(|| reference_images.first().map(RgbImage::height))
            .unwrap_or(1024);
        let width = options
            .width
            .or_else(|| reference_images.first().map(RgbImage::width))
            .unwrap_or(1024);
        let height = QwenImageProcessor::align(height);
        let width = QwenImageProcessor::align(width);
        ensure!(width > 0 && height > 0);

        self.model.forward(&ImageGenerationParams {
            prompt: prompt.to_owned(),
            width: i32::try_from(width)?,
            height: i32::try_from(height)?,
            reference_images,
            sample: SampleParams {
                guidance: GuidanceParams {
                    text_cfg: CFG_SCALE,
                    ..GuidanceParams::default()
                },
                sample_method: SampleMethod::Euler,
                sample_steps: options.num_inference_steps,
                ..SampleParams::default()
            },
            seed: options.seed,
            batch_count: options.num_images_per_prompt,
            vae_tiling: self.model.vae_tiling(),
            ..ImageGenerationParams::default()
=======
            options.num_images_per_prompt > 0 && options.output_resolution > 0,
            "num_images_per_prompt and output_resolution must be greater than zero"
        );
        ensure!(
            options.true_cfg_scale.is_finite(),
            "true_cfg_scale must be finite"
        );
        let area = options.output_resolution * options.output_resolution;
        let (width, height) = match images.last() {
            Some(last) => {
                let (width, height) = dimensions(area, last.width(), last.height());
                (
                    options.width.unwrap_or(width),
                    options.height.unwrap_or(height),
                )
            }
            None => (
                options.width.unwrap_or(options.output_resolution),
                options.height.unwrap_or(options.output_resolution),
            ),
        };
        let (width, height) = (width / MULTIPLE * MULTIPLE, height / MULTIPLE * MULTIPLE);
        ensure!(
            width > 0 && height > 0,
            "height and width must be at least {MULTIPLE}"
        );

        // One resize feeds both the text encoder and the VAE. The images go as
        // RGB, which sd.cpp gives an opaque alpha, as diffusers' `RGBA`
        // conversion does.
        let references = images
            .iter()
            .map(|image| {
                let (width, height) = dimensions(area, image.width(), image.height());
                crate::image::resize(&image.to_rgb8(), width, height, RESIZE)
            })
            .collect::<Result<Vec<RgbImage>>>()?;
        let negative_prompt = options.negative_prompt.as_deref();
        let guided = options.true_cfg_scale > 1.0 && negative_prompt.is_some();
        self.model.forward(&ImageParams {
            prompt,
            negative_prompt: negative_prompt.unwrap_or_default(),
            width: Some(width),
            height: Some(height),
            references: &references,
            cfg: Some(if guided { options.true_cfg_scale } else { 1.0 }),
            sample_method: Some(SampleMethod::Euler),
            steps: Some(STEPS as u32),
            sigmas: &SIGMAS,
            seed: Some(options.seed),
            count: Some(options.num_images_per_prompt),
            ..ImageParams::default()
>>>>>>> upstream/main
        })
    }
}

<<<<<<< HEAD
#[derive(Debug)]
pub struct QwenImageInpaint {
    model: Model,
}

impl QwenImageInpaint {
    /// Loads the upstream checkpoint (`leejet/Qwen-Image-2.1-GGUF`).
    pub async fn load(device: crate::Device) -> Result<Self> {
        Self::load_diffusion(device, DIFFUSION_WEIGHTS).await
    }

    /// Loads the uncensored checkpoint
    /// (`abenzerps/Qwen-Image-2.1-Uncensored-GGUF`): the diffusion weights
    /// drop the base model's content refusals, while the Heretic text encoder
    /// and the shared VAE stay.
    pub async fn load_uncensored(device: crate::Device) -> Result<Self> {
        Self::load_diffusion(device, UNCENSORED_DIFFUSION_WEIGHTS).await
    }

    async fn load_diffusion(
        device: crate::Device,
        diffusion_weights: koharu_runtime::HuggingFaceFile<'static>,
    ) -> Result<Self> {
        let started = std::time::Instant::now();
        let (diffusion_model, text_encoder, text_encoder_vision, vae) = tokio::try_join!(
            diffusion_weights.resolve(),
            TEXT_ENCODER_WEIGHTS.resolve(),
            TEXT_ENCODER_VISION_WEIGHTS.resolve(),
            VAE_WEIGHTS.resolve(),
        )
        .context("failed to resolve Qwen Image 2.1 model assets")?;
        let model = Model::new(
            &device,
            ModelPaths {
                diffusion_model,
                text_encoder,
                text_encoder_vision,
                vae,
            },
        )?;
        tracing::info!(
            elapsed = format!("{:.1}s", started.elapsed().as_secs_f32()),
            "loaded Qwen Image 2.1"
        );
        Ok(Self { model })
    }

    /// Edits `image` inside `mask_image` following `prompt`.
    ///
    /// The edited crop is conditioned on itself as a reference image, matching
    /// upstream's `-r` editing mode, while the native denoise mask keeps the
    /// unmasked latent close to the source.
    pub fn inference(
        &self,
        prompt: &str,
        image: &DynamicImage,
        image_reference: Option<&DynamicImage>,
        mask_image: &DynamicImage,
        options: &QwenImageInpaintOptions,
    ) -> Result<DynamicImage> {
        ensure!(
            image.width() == mask_image.width() && image.height() == mask_image.height(),
            "image/mask dimensions differ: image={}x{}, mask={}x{}",
            image.width(),
            image.height(),
            mask_image.width(),
            mask_image.height()
        );
        ensure!(
            !prompt.contains('\0'),
            "prompt contains an interior NUL byte"
        );
        ensure!(
            options.strength > 0.0 && options.strength <= 1.0,
            "Qwen Image 2.1 inpaint strength must be greater than zero and at most one"
        );
        ensure!(
            options.num_inference_steps > 0,
            "num_inference_steps must be greater than zero"
        );

        let mut image = image.clone();
        if u64::from(image.width()) * u64::from(image.height()) > 1024 * 1024 {
            image = QwenImageProcessor::_resize_to_target_area(&image, 1024 * 1024);
        }
        let width = QwenImageProcessor::align(image.width());
        let height = QwenImageProcessor::align(image.height());
        ensure!(width > 0 && height > 0);
        let source = image.to_rgb8();
        let mut image = RgbImage::new(width, height);
        Resizer::new()
            .resize(
                &source,
                &mut image,
                &ResizeOptions::new()
                    .resize_alg(ResizeAlg::Convolution(FilterType::Lanczos3))
                    .use_alpha(false),
            )
            .expect("source and destination images have the same pixel type");
        let source_mask = mask_image.to_luma8();
        let mut mask_image = image::GrayImage::new(width, height);
        Resizer::new()
            .resize(
                &source_mask,
                &mut mask_image,
                &ResizeOptions::new()
                    .resize_alg(ResizeAlg::Convolution(FilterType::Lanczos3))
                    .use_alpha(false),
            )
            .expect("source and destination masks have the same pixel type");

        let crop_coords = options.padding_mask_crop.and_then(|padding| {
            QwenImageProcessor::get_crop_region(&mask_image, width, height, padding)
        });
        let (init_image, mut native_mask) = if let Some((x1, y1, x2, y2)) = crop_coords {
            let image_crop = DynamicImage::ImageRgb8(
                image::imageops::crop_imm(&image, x1, y1, x2 - x1, y2 - y1).to_image(),
            );
            let mask_crop = DynamicImage::ImageLuma8(
                image::imageops::crop_imm(&mask_image, x1, y1, x2 - x1, y2 - y1).to_image(),
            );
            (
                QwenImageProcessor::_resize_and_fill(&image_crop, width, height).to_rgb8(),
                QwenImageProcessor::_resize_and_fill(&mask_crop, width, height).to_luma8(),
            )
        } else {
            (image.clone(), mask_image.clone())
        };
        QwenImageProcessor::binarize(&mut native_mask);

        let mut reference_images = vec![init_image.clone()];
        if let Some(image_reference) = image_reference {
            QwenImageProcessor::check_image_input(image_reference)?;
            reference_images.push(image_reference.clone().to_rgb8());
        }

        let strength = if options.strength >= 1.0 {
            1.0
        } else {
            let effective_steps = options.num_inference_steps
                - (options.num_inference_steps as f64 * (1.0 - options.strength)).floor() as usize;
            let boundary = effective_steps as f32 / options.num_inference_steps as f32;
            f32::from_bits(boundary.to_bits() - 1)
        };

        let generated = self
            .model
            .forward(&ImageGenerationParams {
                prompt: prompt.to_owned(),
                width: i32::try_from(width)?,
                height: i32::try_from(height)?,
                init_image: Some(init_image),
                reference_images,
                mask_image: Some(native_mask),
                sample: SampleParams {
                    guidance: GuidanceParams {
                        text_cfg: CFG_SCALE,
                        ..GuidanceParams::default()
                    },
                    sample_method: SampleMethod::Euler,
                    sample_steps: i32::try_from(options.num_inference_steps)?,
                    ..SampleParams::default()
                },
                seed: options.seed,
                batch_count: 1,
                strength,
                vae_tiling: self.model.vae_tiling(),
                ..ImageGenerationParams::default()
            })?
            .into_iter()
            .next()
            .context("Qwen Image 2.1 returned no inpainted image")?;

        // Qwen Image 2.1 emits RGBA; transparent texels fall back to the source
        // before the mask overlay blends the rest.
        let generated = QwenImageProcessor::flatten_over_source(&generated, &image)?;
        let generated =
            QwenImageProcessor::apply_overlay(&mask_image, &image, generated, crop_coords)?;
        Ok(DynamicImage::ImageRgb8(generated))
    }
=======
/// `calculate_dimensions`: `target_area` at the image's aspect ratio, each
/// side rounded to a multiple of 32 as Python's `round` rounds, half to even.
fn dimensions(target_area: u32, image_width: u32, image_height: u32) -> (u32, u32) {
    let ratio = f64::from(image_width) / f64::from(image_height);
    let width = (f64::from(target_area) * ratio).sqrt();
    let height = width / ratio;
    let round = |side: f64| ((side / f64::from(MULTIPLE)).round_ties_even() as u32) * MULTIPLE;
    (round(width), round(height))
>>>>>>> upstream/main
}
