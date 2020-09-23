#![allow(unused)]

use crate::html;
use crate::layout_engine;
use printpdf::*;

pub struct Engine {
    elements: layout_engine::LayoutResult,
    pages: Vec<DrawTargetPage>
}

struct DrawTargetPage {
    page: PdfPageReference,
    layer: PdfLayerReference,
}

impl Engine {
    fn add_page(&mut self, pdf: &PdfDocumentReference) -> &DrawTargetPage {
        let (index, layer_index) = pdf.add_page(Mm(210.0), Mm(247.0), "MainLayer");
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
        let page = self.add_page(pdf);

        for element in &self.elements {
            self.draw_element(&element, pdf);
        }
    }

    fn draw_element(&self, element: &layout_engine::Element, pdf: &PdfDocumentReference) {
        let page = self.get_page(0);

        // Draw background.
        if element.color == 1 {
            page.layer.set_fill_color(Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None)));
        } else {
            page.layer.set_fill_color(Color::Rgb(Rgb::new(1.0, 0.0, 0.0, None)));
        }

        self.draw_rect(page, element.x, element.y, element.width, element.height);
    }

    fn draw_rect(&self, page: &DrawTargetPage, x: f64, y: f64, width: f64, height: f64) {
        let shape = Line {
            points: vec![
                (Point::new(Mm(x), Mm(self.flip_y(y))), false),
                (Point::new(Mm(x + width), Mm(self.flip_y(y))), false),
                (Point::new(Mm(x + width), Mm(self.flip_y(y + height))), false),
                (Point::new(Mm(x), Mm(self.flip_y(y + height))), false),
            ],
            is_closed: true,
            has_fill: true,
            has_stroke: false,
            is_clipping_path: false
        };

        page.layer.add_shape(shape);
    }

    fn flip_y(&self, y: f64) -> f64 {
        247.0 - y
    }

    pub fn new(doc: layout_engine::LayoutResult) -> Engine {
        Engine {
            elements: doc,
            pages: Vec::new()
        }
    }
}
