//! Koharu's scene graph: Photoshop's layers, a page background, and the parts
//! of a manga page as typed kinds. The README is the design.

mod change;
mod color;
mod edit;
mod error;
mod ids;
mod kinds;
mod layer;
mod project;
mod query;
mod session;
mod store;
mod text;
mod tiles;
mod values;

<<<<<<< HEAD
pub use change::{
    Change, ComponentChange, ComponentOwner, EntityChange, RelationChange, ValueChangeKind,
};
pub use component::{Component, ValidationContext};
pub use components::{
    Asset, AssetInput, AssetMetadata, AssetRole, Authored, DetectionAnalysis, DetectionLabel,
    DocumentProfile, EntityOrigin, FontStyle, Generation, Geometry, Group, LanguageTag,
    OcrAnalysis, Origin, Page, PageDraft, Point, ProfileTerm, Project, RasterLayer,
    RasterLayerKind, Region, RegionKind, Relation, RelationKind, SourceText, TextAlignment,
    TextContent, TextDirection, TextGroup, TextLayout, TextLayoutKind, TextRole, Translation,
    Typography, Visibility, WritingMode,
};
pub use document::{AnalysisRegionRef, GroupRef, TextContentRef, TextLayerRef};
pub use edit::{At, Edit, RemovePolicy};
=======
pub use change::{Change, Created, History, PageChanges};
pub use color::{Color, Rgba};
pub use edit::{Edit, Op};
>>>>>>> upstream/main
pub use error::{Error, Result};
pub use icu_locale_core::LanguageIdentifier;
pub use ids::{Hash, LayerId, PageId, ProjectId, Ref, Revision, Uuid};
pub use kinds::*;
pub use layer::{Attributes, Layer, NewLayer, PixelFormat};
pub use project::{Background, Page, Project};
pub use query::{Answer, PageSummary, ProjectAnswer, Query};
pub use session::{EXTENSION, Session};
pub use text::{CharStyle, RichText, Span, Text, TextStyle};
pub use tiles::{Tile, Tiles};
pub use values::*;
