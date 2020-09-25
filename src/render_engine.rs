#![allow(unused)]

use crate::html;
use crate::layout_engine;
use crate::context;
use crate::resources_manager::{ResourcesManagerRef, FontResource};
use crate::font;
use crate::color;
use printpdf::*;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

pub struct Engine {
    document: html::DocumentRef,
    resource_manager: ResourcesManagerRef,
    elements: layout_engine::LayoutResult,
    pages: Vec<DrawTargetPage>,
    fonts: HashMap<String, IndirectFontRef>,
    fallback_font: Option<IndirectFontRef>
}

struct DrawTargetPage {
    page: PdfPageReference,
    layer: PdfLayerReference,
}

impl Engine {
    fn add_page(&mut self, pdf: &PdfDocumentReference) -> &DrawTargetPage {
        let (index, layer_index) = pdf.add_page(Mm(210.0), Mm(297.0), "MainLayer");
        let page = DrawTargetPage {
            page: pdf.get_page(index),
            layer: pdf.get_page(index).get_layer(layer_index)
        };

        self.pages.push(page);
        return self.pages.last().unwrap();
    }

    fn get_page(&self, index: usize) -> &DrawTargetPage {
        return &self.pages[index];
    }

    pub fn render(&mut self, pdf: &PdfDocumentReference) {

        // Prepare all fonts.
        for font in self.resource_manager.borrow().fonts.values() {
            self.fonts.insert(font.name.clone(), pdf.add_external_font(&font.data).unwrap());
        }

        self.fallback_font = Some(pdf.add_builtin_font(BuiltinFont::TimesRoman).unwrap());

        let page = self.add_page(pdf);

        for element in &self.elements {
            self.draw_element(&element, pdf);
        }
    }

    fn draw_element(&self, element: &layout_engine::Element, pdf: &PdfDocumentReference) {
        let page = self.get_page(0);
        let doc = self.document.borrow();
        let html_element = doc.get_element_immutable(element.element);
        let resource_manager = self.resource_manager.borrow();

        // Draw background.
        if !html_element.is_text_node() {
            self.draw_rect(page, element.x, element.y, element.width, element.height);
        }

        // Draw the text, if this is a text node.
        if html_element.is_text_node() {
            let fallback_font = self.fallback_font.as_ref().unwrap();
            let font_name = self.get_font_name(&html_element);

            let default_color_code = color::get_default_color_code();
            let text_color_code = doc.get_element_style_property(element.element, "color").unwrap_or(&default_color_code);
            let text_color = color::code_to_color(text_color_code);

            page.layer.set_fill_color(self.color_to_printpdf_color(&text_color));

            if let Some(external_font) = self.fonts.get(&font_name) {
                let font_resource = &resource_manager.get_font(&font_name).unwrap().font;
                let text_bb = font_resource.get_text_bounding_box(&html_element.text, 12.0, true);
                self.draw_text(page, element.x, element.y + text_bb.height - text_bb.y, &html_element.text, &external_font);
            } else {
                self.draw_text(page, element.x, element.y, &html_element.text, &fallback_font);
            }
        }
    }

    fn draw_rect(&self, page: &DrawTargetPage, x: f64, y: f64, width: f64, height: f64) {
        let ww = self.px_to_mm(width);
        let hh = self.px_to_mm(height);
        let xx = self.px_to_mm(x);
        let yy = self.px_to_mm(y);

        let shape = Line {
            points: vec![
                (Point::new(Mm(xx), Mm(self.flip_y(yy))), false),
                (Point::new(Mm(xx + ww), Mm(self.flip_y(yy))), false),
                (Point::new(Mm(xx + ww), Mm(self.flip_y(yy + hh))), false),
                (Point::new(Mm(xx), Mm(self.flip_y(yy + hh))), false),
            ],
            is_closed: true,
            has_fill: true,
            has_stroke: false,
            is_clipping_path: false
        };

        page.layer.add_shape(shape);
    }

    fn draw_text(&self, page: &DrawTargetPage, x: f64, y: f64, text: &String, font: &IndirectFontRef) {
        let xx = self.px_to_mm(x);
        let yy = self.px_to_mm(y);
        page.layer.use_text(text, 12, Mm(xx), Mm(self.flip_y(yy)), font);
    }

    fn px_to_mm(&self, val: f64) -> f64 {
        let dpi = 72.0;
        let to_mm = 25.4 / dpi;
        return val * to_mm;
    }

    fn flip_y(&self, y: f64) -> f64 {
        297.0 - y
    }

    fn get_font_name(&self, element: &html::Element) -> String {
        if let Some(font_prop) = element.get_style_property("font") {
            return font_prop.clone();
        }

        return String::from("Arial");
    }

    fn color_to_printpdf_color(&self, color: &color::Color) -> printpdf::Color {
        return printpdf::Color::Rgb(Rgb::new(color.red as f64 / 255.0, color.green as f64 / 255.0, color.blue as f64 / 255.0, None));
    }

    pub fn new(context: context::ConversionContext) -> Engine {
        Engine {
            document: context.document,
            elements: context.layout_result.unwrap(),
            resource_manager: context.resources_manager.unwrap(),
            pages: Vec::new(),
            fonts: HashMap::default(),
            fallback_font: None
        }
    }
}
