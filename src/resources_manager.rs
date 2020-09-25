#![allow(unused)]

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use crate::font;

pub type ResourcesManagerRef = Rc<RefCell<ResourcesManager>>;

pub struct ResourcesManager {
    pub fonts: HashMap<String, FontResource>,
    pub images: HashMap<String, ImageResource>,
}

#[derive(Debug)]
pub struct FontResource {
    pub name: String,
    pub font: font::Font,
    pub data: std::fs::File,
}

#[derive(Debug, Default)]
pub struct ImageResource {
    pub path: String
}

impl ResourcesManager {
    pub fn add_font(&mut self, name: String, font: FontResource) {
        self.fonts.insert(name, font);
    }

    pub fn get_font(&self, name: &String) -> Option<&FontResource> {
        return self.fonts.get(name);
    }

    pub fn add_image(&mut self, path: String, image: ImageResource) {
        self.images.insert(path, image);
    }

    pub fn get_image(&self, path: &String) -> Option<&ImageResource> {
        return self.images.get(path);
    }

    pub fn new() -> Self {
        Self {
            fonts: HashMap::default(),
            images: HashMap::default()
        }
    }
}
