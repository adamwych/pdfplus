#![allow(unused)]

use crate::html;
use crate::layout_engine;
use crate::render_engine;
use crate::context;
use crate::png_render_engine;
use printpdf::*;

pub fn generate_pdf(context: context::ConversionContext, render_pdf: bool) -> PdfDocumentReference {
    let pdf = PdfDocument::empty("Title");

    if (render_pdf) {
        let mut renderer = render_engine::Engine::new(context);
        renderer.render(&pdf);
    } else {
        let mut renderer = png_render_engine::Engine::new(context);
        renderer.render();
    }

    return pdf;
}
