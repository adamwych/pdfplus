#![allow(unused)]

use crate::html;
use crate::layout_engine;
use crate::render_engine;
use printpdf::*;

pub fn generate_pdf(document: layout_engine::LayoutResult) -> PdfDocumentReference {
    let pdf = PdfDocument::empty("Title");

    let mut renderer = render_engine::Engine::new(document);
    renderer.render(&pdf);

    return pdf;
}
