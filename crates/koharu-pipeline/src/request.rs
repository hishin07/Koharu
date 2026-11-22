//! What a run is asked to do.

use std::collections::BTreeSet;

use koharu_scene::{LayerId, PageId, Rect};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::{Display, VariantArray};

use crate::PipelineConfig;

/// One run: `stages` on a target, each page's in stage order.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Request {
<<<<<<< HEAD
    pub operation: Operation,
    pub scope: Scope,
    pub stop: StopToken,
    pub progress: Option<ProgressSink>,
    pub inpainting_mask: Option<InpaintingMask>,
    /// Terminology this run must honour, merged into the chapter profile the
    /// translation stage resolves: a pair replaces the term of the same source,
    /// while everything else the analysis learned stays. It travels on the
    /// request rather than in the scene so a run's fixed terms never settle
    /// into the project's dossier.
    pub glossary: Vec<koharu_translator::ProfileTerm>,
}

impl Default for Request {
    fn default() -> Self {
        Self {
            operation: Operation::Full,
            scope: Scope::Project,
            stop: StopToken::default(),
            progress: None,
            inpainting_mask: None,
            glossary: Vec::new(),
        }
    }
=======
    pub target: Target,
    pub stages: BTreeSet<Stage>,
    /// For this run only; the pipeline's own otherwise.
    #[serde(default)]
    pub config: Option<PipelineConfig>,
}

/// What a run works on. It runs every stage it's given on everything it
/// names, replacing what's there, and never a stage it isn't given.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[derive(specta::Type)]
pub enum Target {
    /// Every page.
    Project,
    Pages(Vec<PageId>),
    /// Recognition, inpainting, or translation on blocks.
    Blocks(Vec<LayerId>),
    /// Inpainting only: a new raster layer limited by a layer mask to the
    /// area, as Photoshop's Generative Fill and Content-Aware Fill write one.
    /// Its prompt goes ahead of the inpainter section's; LaMa, which takes
    /// none, ignores it.
    Area {
        page: PageId,
        area: Area,
        prompt: Option<String>,
    },
>>>>>>> upstream/main
}

/// A stage, in the order a page's stages run.
#[derive(
    Clone,
    Copy,
    Debug,
    Display,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
    JsonSchema,
    VariantArray,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
#[derive(specta::Type)]
pub enum Stage {
    Detect,
    Recognize,
    Inpaint,
    Translate,
}

/// A painted area of a page: a byte a pixel over `rect`, in page pixels,
/// painted where it isn't 0.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, specta::Type)]
pub struct Area {
    pub rect: Rect,
    pub pixels: Vec<u8>,
}
