use crate::html;
use crate::layout_engine;
use crate::resources_manager::ResourcesManagerRef;

/// Provides a storage for all conversion-related things.
pub struct ConversionContext {

    /// The HTML document that is being converted.
    pub document: html::DocumentRef,

    /// Output from the layout engine.
    pub root_element: Option<layout_engine::Element>,

    /// External resources manager.
    pub resources_manager: Option<ResourcesManagerRef>,

}
