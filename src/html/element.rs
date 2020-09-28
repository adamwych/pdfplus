#![allow(dead_code)]

use std::collections::HashMap;
use crate::css::{parse_inline, PrimitiveValue};
use crate::html::ElementStyleProperties;

#[derive(Debug, Clone)]
pub struct Element {

    /// Index of this element in document's elements list.
    pub index: usize,

    /// Tag name.
    pub tag: String,

    // Special case for text nodes. Contains the text.
    pub text: String,

    /// Index of this element's parent element.
    pub parent: usize,
    pub has_parent: bool,

    /// List of all direct children of this element.
    pub children: Vec<usize>,

    /// Contains all attributes of this element.
    attributes: HashMap<String, String>,

    /// Contains all style properties.
    /// This list contains only those properties, which are actually active
    /// meaning that if someone overwrites a property from a CSS file using
    /// element's inline `style` attribute, then the latter will be put here.
    style: ElementStyleProperties

}

impl Element {

    /// Adds specified attribute to the node.
    /// An attribute's name must not contains spaces.
    pub fn add_attribute(&mut self, name: &str, value: &str) {
        if name.contains(" ") {
            println!("error: attribute's name must not contain spaces (found \"{}\")", name);
            return;
        }

        self.attributes.insert(String::from(name), String::from(value));
    }

    /// Returns value of specified attribute, if added to the node.
    pub fn get_attribute(&self, name: &str) -> Option<&String> {
        return self.attributes.get(name);
    }

    /// Removes specified attribute from the node.
    pub fn remove_attribute(&mut self, name: &str) {
        self.attributes.remove(name);
    }

    /// Returns whether an attribute with specified name was added
    /// to the node.
    pub fn has_attribute(&self, name: &str) -> bool {
        return self.attributes.contains_key(name);
    }

    pub fn add_default_style_properties(&mut self) {
        let mut default_styles = "";

        match self.tag.as_str() {
            "html" => {
                
            },

            "div" => {
                default_styles = "display: block; color: black;";
            },

            _ => {}
        }

        let styles = parse_inline(default_styles);
        for prop in styles {
            self.style.set_default(&prop.name, prop.value);
        }
    }

    pub fn add_style_property(&mut self, name: &str, value: PrimitiveValue) {
        self.style.set(name, value);
    }

    /// Returns value of specified style property, if added to the node.
    pub fn get_style_property(&self, name: &str) -> Option<&PrimitiveValue> {
        return self.style.get(name);
    }

    pub fn set_style_properties(&mut self, properties: ElementStyleProperties) {
        self.style = properties;
    }

    pub fn get_style_properties(&self) -> &ElementStyleProperties {
        return &self.style;
    }

    /// Returns whether a style property with specified name was
    /// added to the node.
    pub fn has_style_property(&self, name: &str) -> bool {
        return self.style.has(name);
    }

    /// Returns whether this is a text node.
    pub fn is_text_node(&self) -> bool {
        return self.tag == "#text";
    }

    pub fn default() -> Element {
        Element {
            index: 0,
            tag: String::default(),
            text: String::default(),
            parent: 0,
            has_parent: false,
            children: Vec::new(),
            attributes: HashMap::new(),
            style: ElementStyleProperties::new()
        }
    }
}
