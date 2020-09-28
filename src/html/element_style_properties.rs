#![allow(dead_code)]

use std::collections::HashMap;
use crate::css::PrimitiveValue;

#[derive(Debug, Clone)]
pub struct ElementStyleProperties {
    properties: HashMap<String, PrimitiveValue>,
    default_properties: HashMap<String, PrimitiveValue>
}

impl ElementStyleProperties {
    pub fn set(&mut self, name: &str, value: PrimitiveValue) {
        self.properties.insert(name.to_string(), value);
    }

    pub fn set_default(&mut self, name: &str, value: PrimitiveValue) {
        self.default_properties.insert(name.to_string(), value);
    }

    pub fn get(&self, name: &str) -> Option<&PrimitiveValue> {
        if self.properties.contains_key(name) {
            return self.properties.get(name);
        }

        return self.default_properties.get(name);
    }

    pub fn get_default(&self, name: &str) -> Option<&PrimitiveValue> {
        return self.default_properties.get(name);
    }

    pub fn has(&self, name: &str) -> bool {
        return self.properties.contains_key(name);
    }

    pub fn has_default(&self, name: &str) -> bool {
        return self.default_properties.contains_key(name);
    }

    pub fn get_entries(&self) -> HashMap<String, PrimitiveValue> {
        return self.properties.clone();
    }

    pub fn get_default_entries(&self) -> HashMap<String, PrimitiveValue> {
        return self.default_properties.clone();
    }

    /// Merges two style properties and returns the result.
    pub fn merge(child: &ElementStyleProperties, parent: &ElementStyleProperties) -> ElementStyleProperties {
        let mut result = ElementStyleProperties::new();

        for (name, value) in parent.get_entries() {
            if can_be_inherited(&name) {
                result.set(&name, value);
            }
        }

        for (name, value) in child.get_entries() {
            result.set(&name, value);
        }

        for (name, value) in parent.get_default_entries() {
            if can_be_inherited(&name) {
                if !result.has(&name) {
                    result.set(&name, value);
                }
            }
        }

        for (name, value) in child.get_default_entries() {
            if !result.has(&name) {
                result.set(&name, value);
            }
        }

        return result;
    }

    pub fn new() -> Self {
        Self {
            properties: HashMap::new(),
            default_properties: HashMap::new()
        }
    }
}

lazy_static! {
    // https://drafts.csswg.org/css2/#property-index
    static ref INHERITED_PROPERTIES: Vec<&'static str> = vec![
        "border-collapse",
        "border-spacing",
        "caption-side",
        "color",
        "cursor",
        "direction",
        "empty-cells",
        "font",
        "font-family",
        "font-size",
        "font-style",
        "font-variant",
        "font-weight",
        "letter-spacing",
        "line-height",
        "list-style",
        "list-style-image",
        "list-style-position",
        "list-style-type",
        "orphans",
        "quotes",
        "text-align",
        "text-ident",
        "text-transform",
        "visibility",
        "white-space",
        "widows",
        "word-spacing"
    ];
}

fn can_be_inherited(name: &str) -> bool {
    return INHERITED_PROPERTIES.contains(&name);
}