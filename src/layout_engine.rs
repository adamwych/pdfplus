#![allow(unused)]

use crate::html;

pub struct Engine {
    document: html::Document,
}

// Contains the result of layout engine's calculations.
#[derive(Debug)]
pub struct Element {
    pub element: usize,
    pub width: f64,
    pub height: f64,
    pub x: f64,
    pub y: f64,
    pub color: i32,
}

pub type LayoutResult = Vec<Element>;

impl Engine {
    pub fn process_document(&self) -> LayoutResult {
        let mut result = self.process_element(self.document.get_root_immutable());
        println!("{} elements layed out", result.len());
        return result;
    }

    /// Calculates layout properties of a generic element.
    fn process_element(&self, element: &html::Element) -> Vec<Element> {
        if element.children.len() == 0 {
            return vec![self.process_lonely_element(element)];
        }

        let mut result = Vec::<Element>::new();
        let mut elem = Element {
            color: 1,
            ..Element::default(element.index)
        };

        let target_position = result.len();

        for element_idx in &element.children {
            let element = self.document.get_element_immutable(*element_idx);
            let mut results = self.process_element(element);

            result.append(&mut results);
        }

        // Calculate positions of element's children.
        for x in 1..result.len() {
            let previous_element = &result[x - 1];
            let y = previous_element.y + previous_element.height;
            result[x].y += y;
        }

        // Calculate width and height of the element itself.
        let mut width: f64 = 0.0;
        let mut height: f64 = 0.0;

        for x in 0..result.len() {
            let element = &result[x];
            width = width.max(result[x].width);
            height += element.height;
        }

        elem.width = width;
        elem.height = height;

        self.clamp_element_size(&element, &mut elem);

        result.insert(target_position, elem);

        return result;
    }

    /// Calculates layout properties of an element with no children
    /// and returns it.
    fn process_lonely_element(&self, element: &html::Element) -> Element {
        let mut width = 0.0;
        let mut height = 0.0;

        if element.has_style_property("width") {
            width = element.get_style_property("width").unwrap().parse().unwrap();
        }

        if element.has_style_property("height") {
            height = element.get_style_property("height").unwrap().parse().unwrap();
        }

        return Element {
            width: width,
            height: height,
            color: 0,
            ..Element::default(element.index)
        }
    }

    /// Clamps given layout element's width and height to be within the range
    /// specified by HTML element's [min-width, max-width] and [min-height, max-height]
    /// style properties.
    fn clamp_element_size(&self, html_element: &html::Element, element: &mut Element) {
        let mut min_width = 0.0;
        let mut max_width = std::f64::INFINITY;

        let mut min_height = 0.0;
        let mut max_height = std::f64::INFINITY;

        if let Some(min_width_prop) = html_element.get_style_property("min-width") {
            min_width = min_width_prop.parse().unwrap();
        }

        if let Some(max_width_prop) = html_element.get_style_property("max-width") {
            max_width = max_width_prop.parse().unwrap();
        }

        if let Some(min_height_prop) = html_element.get_style_property("min-height") {
            min_height = min_height_prop.parse().unwrap();
        }

        if let Some(max_height_prop) = html_element.get_style_property("max-height") {
            max_height = max_height_prop.parse().unwrap();
        }

        element.width = clamp(element.width, min_width, max_width);
        element.height = clamp(element.height, min_height, max_height);
    }

    pub fn new(document: html::Document) -> Engine {
        Engine {
            document: document
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
            color: 0
        }
    }
}

/// Clamps given value to be within given range.
fn clamp(val: f64, min: f64, max: f64) -> f64 {
    if val < min {
        return min;
    }

    if val > max {
        return max;
    }

    return val;
}