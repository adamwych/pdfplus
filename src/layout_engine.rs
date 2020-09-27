#![allow(unused)]

use std::ops::IndexMut;
use std::rc::Rc;
use std::cell::RefCell;
use crate::html;
use crate::resources_manager::{ResourcesManagerRef, FontResource};

pub struct Engine {
    document: html::DocumentRef,
    resource_manager: ResourcesManagerRef
}

/// Represents the result of layout calculations for a single HTML element.
#[derive(Debug, Clone)]
pub struct Element {

    /// Index of the corresponding HTML element.
    pub element: usize,

    /// Final width of the element, taking into account its children and display properties.
    pub width: f64,

    /// Final height of the element, taking into account its children and display properties.
    pub height: f64,

    /// Position on the X axis, accounting for the position of the parent.
    pub x: f64,

    /// Position on the Y axis, accounting for the position of the parent.
    pub y: f64,
    
    /// Position on the X axis, not accounting for the position of the parent.
    pub local_x: f64,

    /// Position on the Y axis, not accounting for the position of the parent.
    pub local_y: f64,

    /// List of this element's direct children handles.
    pub children: Vec<Element>,
}

#[derive(Debug, Clone)]
pub struct ElementHandle {
    index: usize
}

impl Engine {
    pub fn process_document(&self) -> Element {
        let doc = self.document.borrow();
        let root = doc.get_root_immutable();
        let mut root_element = self.process_element(root, None);
        root_element.children = self.adjust_children_position(&root_element, root_element.children.clone());
        return root_element;
    }

    /// Calculates layout properties of a generic element.
    fn process_element(&self, element: &html::Element, parent: Option<&Element>) -> Element {
        if element.children.len() == 0 {
            return self.process_lonely_element(element, parent);
        }

        return self.process_crowded_element(element, parent);
    }

    /// Calculates layout properties of an element with no children.
    fn process_lonely_element(&self, element: &html::Element, parent: Option<&Element>) -> Element {
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
    fn process_crowded_element(&self, element: &html::Element, parent: Option<&Element>) -> Element {
        let doc = self.document.borrow();
        let mut elem = Element::default(element.index);
        let mut children = Vec::<Element>::new();

        for element_idx in &element.children {
            let element = doc.get_element_immutable(*element_idx);
            let mut child = self.process_element(element, Some(&elem));
            children.push(child);
        }

        let mut children_clone = children.clone();
        let children_num = children_clone.len();

        // Calculate positions of element's children.
        // todo: Change this to avoid cloning children on every loop!!
        for idx in 1..children_num {
            let mut child = children.index_mut(idx);
            let previous_child = &children_clone[idx - 1];

            let y = previous_child.local_y + previous_child.height;
            child.local_y += y;
            children_clone = children.clone();
        }

        // Calculate width and height of the element itself.
        let mut width: f64 = 0.0;
        let mut height: f64 = 0.0;

        for child in &children {
            width = width.max(child.local_x + child.width);
            height = height.max(child.local_y + child.height);
        }

        elem.width = width;
        elem.height = height;
        elem.children = children;

        self.clamp_element_size(&element, &mut elem);
        self.adjust_element_position(&element, &mut elem);

        if let Some(parent_element) = parent {
            elem.x += parent_element.x;
            elem.y += parent_element.y;
        } else {
            elem.x = elem.local_x;
            elem.y = elem.local_y;
        }

        return elem;
    }

    /// Clamps given layout element's width and height to be within the range
    /// specified by HTML element's [min-width, max-width] and [min-height, max-height]
    /// style properties.
    fn clamp_element_size(&self, html_element: &html::Element, element: &mut Element) {
        fn get_and_parse_or(html_element: &html::Element, name: &str, default: f64) -> f64 {
            if let Some(prop_value) = html_element.get_style_property(name) {
                return prop_value.as_dimension_value().value;
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

    /// Moves the element according to its `top` and `left` style properties.
    fn adjust_element_position(&self, html_element: &html::Element, element: &mut Element) {
        if let Some(left_prop) = html_element.get_style_property("left") {
            let left: f64 = left_prop.as_dimension_value().value;
            element.local_x += left;
        }

        if let Some(top_prop) = html_element.get_style_property("top") {
            let top: f64 = top_prop.as_dimension_value().value;
            element.local_y += top;
        }
    }

    /// Calculates final position of given elements. Uses `parent` as the origin.
    /// Calls this method for all elements in the tree recursively.
    fn adjust_children_position(&self, parent: &Element, children: Vec<Element>) -> Vec<Element> {
        let mut result = children.clone();

        for idx in 0..children.len() {
            result[idx].x = parent.x + result[idx].local_x;
            result[idx].y = parent.y + result[idx].local_y;
            result[idx].children = self.adjust_children_position(&result[idx], result[idx].children.clone());
        }

        return result;
    }

    fn get_font_name(&self, element: &html::Element) -> String {
        if let Some(font_prop) = element.get_style_property("font") {
            return font_prop.as_string().clone();
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
            y: 0.0,
            local_x: 0.0,
            local_y: 0.0,
            children: Vec::new()
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