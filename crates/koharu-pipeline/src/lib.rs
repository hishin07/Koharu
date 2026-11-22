//! Koharu's models run over a project's pages: every stage on every page in
//! one click, or one stage by hand on a page, a block, or a painted area. The
//! README is the design.

<<<<<<< HEAD
mod accelerator;
pub mod batch;
=======
>>>>>>> upstream/main
mod config;
mod error;
mod event;
mod gpu;
mod pipeline;
mod request;
mod scheduler;
mod stages;

pub use config::PipelineConfig;
pub use error::{Error, Result};
pub use event::{Event, Outcome, RunId};
pub use pipeline::Pipeline;
<<<<<<< HEAD
pub use progress::{Progress, ProgressSink};
pub use report::{Committer, Report, RunStatus, StageOutput};
pub use request::{InpaintingMask, Operation, Request, StopToken};
pub use resources::{DeviceResources, ResourceSnapshot};
pub use scope::{Bounds, Scope};
pub use stage::Stage;
pub use stages::{
    Flux2KleinConfig, KoharuLayoutRFDetrSeg2XLConfig, QwenImageConfig, RoremMixedConfig,
};

use images::ImageCache;
use model_cell::ModelCell;

pub mod vram;

#[cfg(test)]
mod tests;
=======
pub use request::{Area, Request, Stage, Target};
pub use stages::{
    detect::{Detector, DetectorConfig},
    inpaint::{Inpainter, InpainterConfig},
    recognize::{Recognizer, RecognizerConfig},
    translate::{TranslatorConfig, placement},
};
>>>>>>> upstream/main
