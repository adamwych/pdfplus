#![allow(unused)]

use crate::html;
use crate::generator::context;
use crate::generator::render_engine;
use printpdf::*;

pub fn generate_pdf(context: context::ConversionContext) -> PdfDocumentReference {
    let pdf = PdfDocument::empty("Title");

    let mut renderer = render_engine::Engine::new(context);
    renderer.render(&pdf);

    return pdf;
}
