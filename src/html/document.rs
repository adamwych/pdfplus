#![allow(dead_code)]

use std::ops::IndexMut;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use crate::css::PrimitiveValue;

pub type DocumentRef = Rc<RefCell<Document>>;

#[derive(Debug, Clone)]
pub struct Document {

    /// Index of the root element in the `elements` list.
    /// There can only be 1 root element and all other elements
    /// have to be its direct or indirect children.
    root: usize,

    /// A flat list of all elements that have ever been created
    /// by this document.
    elements: Vec<Element>,
    
}

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
    style_properties: HashMap<String, PrimitiveValue>,

}

impl Document {

    /// Sets element at `element_index` to be a child of element at `parent_index`.
    pub fn add_element(&mut self, element_index: usize, parent_index: usize) {
        assert!(element_index != parent_index, "circular reference encountered");
        assert!(element_index < self.elements.len(), "attempted to add an unknown element");
        assert!(parent_index < self.elements.len(), "attempted to add an element to an unknown parent");

        let parent = self.get_element(parent_index);
        if parent.children.contains(&element_index) {
            println!("warning: attempted to add an element to its parent multiple times");
            return;
        }

        parent.children.push(element_index);

        let mut child = self.get_element(element_index);
        child.parent = parent_index;
        child.has_parent = true;
    }

    pub fn add_element_to_root(&mut self, element_index: usize) {
        self.add_element(element_index, self.get_root_index());
    }

    /// Attempts to find the value of specified style property in given element or one of its ancestors.
    pub fn get_element_style_property(&self, element_index: usize, property_name: &str) -> Option<&PrimitiveValue> {
        let mut element = Some(self.get_element_immutable(element_index));

        while element.is_some() {
            let elem = element.unwrap();

            if let Some(prop) = elem.get_style_property(property_name) {
                return Some(prop);
            }

            if elem.has_parent {
                element = Some(self.get_element_immutable(elem.parent));
            } else {
                element = None;
            }
        }

        return None;
    }

    /// Creates a new Element and returns its index.
    pub fn create_element(&mut self, tag: &str) -> usize {
        let idx = self.elements.len();
        let element = Element {
            index: idx,
            tag: String::from(tag),
            ..Element::default()
        };

        self.elements.push(element);
        return idx;
    }

    /// Creates a new text Element and returns its index.
    pub fn create_text_element(&mut self, text: &str) -> usize {
        let idx = self.elements.len();
        let element = Element {
            index: idx,
            tag: String::from("#text"),
            text: String::from(text),
            ..Element::default()
        };

        self.elements.push(element);
        return idx;
    }

    /// Returns element at specified index.
    pub fn get_element(&mut self, index: usize) -> &mut Element {
        return self.elements.index_mut(index);
    }

    /// Returns element at specified index.
    pub fn get_element_immutable(&self, index: usize) -> &Element {
        return &self.elements[index];
    }

    pub fn get_elements(&self) -> std::slice::Iter<Element> {
        return self.elements.iter();
    }

    /// Returns the index of the root element.
    pub fn get_root_index(&self) -> usize {
        return self.root;
    }

    /// Returns the root element.
    pub fn get_root(&mut self) -> &mut Element {
        return self.get_element(self.get_root_index());
    }

    pub fn get_root_immutable(&self) -> &Element {
        return self.get_element_immutable(self.get_root_index());
    }

    /// Creates a new, empty Document.
    pub fn new() -> Rc<RefCell<Document>> {
        let mut document = Document {
            root: 0,
            elements: Vec::new()
        };

        document.create_element("root");

        return Rc::new(RefCell::new(document))
    }
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

    pub fn add_style_property(&mut self, name: &str, value: PrimitiveValue) {
        self.style_properties.insert(String::from(name), value);
    }

    /// Returns value of specified style property, if added to the node.
    pub fn get_style_property(&self, name: &str) -> Option<&PrimitiveValue> {
        return self.style_properties.get(name);
    }

    /// Returns whether a style property with specified name was
    /// added to the node.
    pub fn has_style_property(&self, name: &str) -> bool {
        return self.style_properties.contains_key(name);
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
            style_properties: HashMap::new()
        }
    }
}
