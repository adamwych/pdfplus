use std::rc::Rc;
use std::cell::RefCell;
use font_kit;
use crate::html;
use crate::resources_manager::{ResourcesManager, ResourcesManagerRef, FontResource};
use crate::font;

/// Preprocessor's responsibility is to go through each node in an HTML document
/// and load all external resources like fonts, images etc. so they can be immediately used
/// by the layout and render engines.
pub struct Preprocessor {
    document: html::DocumentRef
}

impl Preprocessor {
    pub fn process_document(&self) -> ResourcesManagerRef {
        let mut manager = Rc::new(RefCell::new(ResourcesManager::new()));
        let document = self.document.borrow();
        self.process_element(document.get_root_immutable(), &mut manager);
        return manager;
    }

    fn process_element(&self, element: &html::Element, manager: &mut ResourcesManagerRef) {
        let document = self.document.borrow();

        if element.is_text_node() {
            let font_name = self.get_font_name(element);
            let mut font_file_path = String::from("");
            font_file_path.push_str(".\\test_font\\");
            font_file_path.push_str(&font_name);
            font_file_path.push_str(".ttf");

            if let Ok(_font) = font_kit::loaders::default::Font::from_path(&font_file_path, 0) {
                manager.borrow_mut().add_font(font_name.clone(), FontResource {
                    name: font_name,
                    font: font::Font::wrap(_font),
                    data: std::fs::File::open(font_file_path).unwrap()
                });
            } else {
                println!("error: unable to load font {}", font_file_path);
            }
        }

        for child_index in &element.children {
            self.process_element(document.get_element_immutable(*child_index), manager);
        }
    }

    fn get_font_name(&self, element: &html::Element) -> String {
        if let Some(font_prop) = element.get_style_property("font") {
            return font_prop.clone();
        }

        return String::from("Arial");
    }

    pub fn new(document: Rc<RefCell<html::Document>>) -> Preprocessor {
        Preprocessor {
            document: document
        }
    }
}