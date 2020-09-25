#![allow(unused)]

use crate::html;
use crate::layout_engine;
use crate::render_engine;
use crate::context;
use crate::png_render_engine;
use printpdf::*;

pub fn generate_pdf(context: context::ConversionContext) -> PdfDocumentReference {
    let pdf = PdfDocument::empty("Title");

    let mut renderer = render_engine::Engine::new(context);
    renderer.render(&pdf);

    return pdf;
}
