#![allow(unused)]

use crate::html;
use crate::layout_engine;
use crate::context;
use crate::resources_manager::{ResourcesManagerRef, FontResource};
use crate::font;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use png;
use std::path::Path;
use std::fs::File;
use std::io::BufWriter;

pub struct Engine {
    document: html::DocumentRef,
    resource_manager: ResourcesManagerRef,
    elements: layout_engine::LayoutResult,
    data: Vec<u8>,
    i: i32
}

impl Engine {
    pub fn render(&mut self) {
        let path = Path::new(r"./test.png");
        let file = File::create(path).unwrap();
        let ref mut w = BufWriter::new(file);
        let mut encoder = png::Encoder::new(w, 595, 842);
        encoder.set_color(png::ColorType::RGB);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();

        self.data.resize(595 * 842 * 3, 255);
    
        for index in 0..self.elements.len() {
            self.draw_element(index);
        }

        writer.write_image_data(&self.data).unwrap();
    }

    fn draw_element(&mut self, idx: usize) {
        let element = &self.elements[idx];
        let doc_c = Rc::clone(&self.document);
        let doc = doc_c.borrow();
        let html_element = doc.get_element_immutable(element.element);
        if !html_element.is_text_node() {
            let x = element.x;
            let y = element.y;
            let width = element.width;
            let height = element.height;
            self.draw_rect(x, y, width, height);
        }
    }

    fn draw_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        for xx in 0..(width as i32) {
            for yy in 0..(height as i32) {
                self.set_pixel(xx as f64 + x, yy as f64 + y, 255, 0, 0);
            }
        }
    }

    fn set_pixel(&mut self, x: f64, y: f64, r: u8, g: u8, b: u8) {
        let idx = ((x % 595.0) as i32) + ((y as i32) * (595 as i32));
        let pos = (idx * 3) as usize;

        self.data[pos] = r;
        self.data[pos + 1] = g;
        self.data[pos + 2] = b;
    }

    fn draw_text(&mut self, x: f64, y: f64, text: &String) {
        
    }

    pub fn new(context: context::ConversionContext) -> Self {
        Self {
            document: context.document,
            elements: context.layout_result.unwrap(),
            resource_manager: context.resources_manager.unwrap(),
            data: Vec::new(),
            i: 0
        }
    }
}
