#![allow(unused)]

use std::ops::IndexMut;
use std::rc::Rc;
use std::cell::RefCell;
use crate::html;
use crate::resources_manager::{ResourcesManagerRef, FontResource};

pub struct Engine {
    document: html::DocumentRef,
    resource_manager: ResourcesManagerRef,
}

// Contains the result of layout engine's calculations.
#[derive(Debug)]
pub struct Element {
    pub element: usize,
    pub width: f64,
    pub height: f64,
    pub x: f64,
    pub y: f64
}

pub type LayoutResult = Vec<Element>;

impl Engine {
    pub fn process_document(&self) -> LayoutResult {
        let mut result = self.process_element(self.document.borrow().get_root_immutable());
        println!("{} elements layed out", result.len());
        return result;
    }

    /// Calculates layout properties of a generic element.
    fn process_element(&self, element: &html::Element) -> LayoutResult {
        if element.children.len() == 0 {
            return vec![self.process_lonely_element(element)];
        }

        return self.process_crowded_element(element);
    }

    /// Calculates layout properties of an element with no children.
    fn process_lonely_element(&self, element: &html::Element) -> Element {
        let mut elem = Element::default(element.index);

        if element.is_text_node() {
            let line_height = 0.0;
            let resource_man = self.resource_manager.borrow();
            if let Some(font) = resource_man.get_font(&self.get_font_name(&element)) {
                let text_box = font.font.get_text_bounding_box(&element.text, 12.0, true);
                elem.width = text_box.width;
                elem.height = text_box.height + line_height;
            }
        }

        self.clamp_element_size(&element, &mut elem);
        self.adjust_element_position(&element, &mut elem);
        
        return elem;
    }

    /// Calculates layout properties of an element that has some children.
    fn process_crowded_element(&self, element: &html::Element) -> LayoutResult {
        let mut result = LayoutResult::new();
        let mut elem = Element::default(element.index);

        let target_position = result.len();

        let doc = self.document.borrow();
        for element_idx in &element.children {
            let element = doc.get_element_immutable(*element_idx);
            let mut results = self.process_element(element);

            result.append(&mut results);
        }

        // Calculate positions of element's children.
        for idx in 1..element.children.len() {
            let previous_element = &result[idx - 1];
            let y = previous_element.y + previous_element.height;
            result[idx].y += y;
        }

        // Calculate width and height of the element itself.
        let mut width: f64 = 0.0;
        let mut height: f64 = 0.0;

        for idx in 0..element.children.len() {
            let child = &result[idx];
            width = width.max(child.x + child.width);
            height = height.max(child.y + child.height);
        }

        elem.width = width;
        elem.height = height;

        self.clamp_element_size(&element, &mut elem);
        self.adjust_element_position(&element, &mut elem);
        self.adjust_children_position(&elem, &mut result);

        result.insert(target_position, elem);

        return result;
    }

    /// Clamps given layout element's width and height to be within the range
    /// specified by HTML element's [min-width, max-width] and [min-height, max-height]
    /// style properties.
    fn clamp_element_size(&self, html_element: &html::Element, element: &mut Element) {
        fn get_and_parse_or(html_element: &html::Element, name: &str, default: f64) -> f64 {
            if let Some(prop_value) = html_element.get_style_property(name) {
                return prop_value.parse().unwrap();
            }
            
            return default;
        }

        // width and height effectively work like min-width and min-height in this context,
        // so first try them, and if they are not set, then try min-width and min-height.

        let min_width = get_and_parse_or(html_element, "width", get_and_parse_or(html_element, "min-width", 0.0));
        let max_width = get_and_parse_or(html_element, "max-width", std::f64::INFINITY);
        
        let min_height = get_and_parse_or(html_element, "height", get_and_parse_or(html_element, "min-height", 0.0));
        let max_height = get_and_parse_or(html_element, "max-height", std::f64::INFINITY);
        
        element.width = clamp(element.width, min_width, max_width);
        element.height = clamp(element.height, min_height, max_height);
    }

    /// Moves the element accordingly to its `top` or `left` style properties.
    fn adjust_element_position(&self, html_element: &html::Element, element: &mut Element) {
        if let Some(left_prop) = html_element.get_style_property("left") {
            let left: f64 = left_prop.parse().unwrap();
            element.x += left;
        }

        if let Some(top_prop) = html_element.get_style_property("top") {
            let top: f64 = top_prop.parse().unwrap();
            element.y += top;
        }
    }

    /// Adjusts element's children position to account for their parent's position.
    fn adjust_children_position(&self, parent: &Element, children: &mut LayoutResult) {
        for idx in 0..children.len() {
            let mut child = children.index_mut(idx);
            child.x += parent.x;
            child.y += parent.y;
        }
    }

    fn get_font_name(&self, element: &html::Element) -> String {
        if let Some(font_prop) = element.get_style_property("font") {
            return font_prop.clone();
        }

        return String::from("Arial");
    }

    pub fn new(document: html::DocumentRef, resource_manager: ResourcesManagerRef) -> Engine {
        Engine {
            document: document,
            resource_manager: resource_manager
        }
    }
}

impl Element {
    pub fn default(element_index: usize) -> Element {
        Element {
            element: element_index,
            width: 0.0,
            height: 0.0,
            x: 0.0,
            y: 0.0
        }
    }
}

/// Clamps given value to given range.
fn clamp(val: f64, min: f64, max: f64) -> f64 {
    if val < min {
        return min;
    }

    if val > max {
        return max;
    }

    return val;
}