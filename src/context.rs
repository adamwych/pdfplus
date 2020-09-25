use crate::html;
use crate::layout_engine;
use crate::resources_manager::ResourcesManagerRef;

/// Provides a storage for all conversion-related things.
pub struct ConversionContext {

    /// The HTML document that is being converted.
    pub document: html::DocumentRef,

    /// Results of layout engine's calculations.
    pub layout_result: Option<layout_engine::LayoutResult>,

    /// External resources manager.
    pub resources_manager: Option<ResourcesManagerRef>,

}
