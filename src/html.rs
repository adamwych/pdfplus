#![allow(dead_code)]

use std::ops::IndexMut;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Document {

    /// Index of the root element in the `elements` list.
    /// There can only be 1 root element and all other elements
    /// have to be its direct or indirect children.
    root: usize,

    /// A flat list of all elements that have ever been created
    /// by this document.
    elements: Vec<Element>,
    
}

#[derive(Debug)]
pub struct Element {

    /// Index of this element in document's elements list.
    pub index: usize,

    /// Tag name.
    pub tag: String,

    /// Index of this element's parent element.
    pub parent: usize,

    /// List of all direct children of this element.
    pub children: Vec<usize>,

    /// Contains all attributes of this element.
    attributes: HashMap<String, String>,

    /// Contains all style properties.
    /// This list contains only those properties, which are actually active
    /// meaning that if someone overwrites a property from a CSS file using
    /// element's inline `style` attribute, then the latter will be put here.
    style_properties: HashMap<String, String>,

}

impl Document {

    /// Sets element at `element_index` to be a child of element at `parent_index`.
    pub fn add_element(&mut self, element_index: usize, parent_index: usize) {
        assert!(element_index < self.elements.len(), "attempted to add an unknown element");
        assert!(parent_index < self.elements.len(), "attempted to add an element to an unknown parent");

        let mut child = self.get_element(element_index);
        child.parent = parent_index;

        let parent = self.get_element(parent_index);
        parent.children.push(element_index);
    }

    pub fn add_element_to_root(&mut self, element_index: usize) {
        self.add_element(element_index, self.get_root_index());
    }

    /// Creates a new Element and returns its index.
    pub fn create_element(&mut self, tag: &str) -> usize {
        let idx = self.elements.len();
        let element = Element {
            index: idx,
            tag: String::from(tag),
            parent: 0,
            children: Vec::new(),
            attributes: HashMap::new(),
            style_properties: HashMap::new()
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
    pub fn new() -> Document {
        let mut document = Document {
            root: 0,
            elements: Vec::new()
        };

        document.create_element("root");

        return document
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

    /// todo: this is here just for easy testing, it should be removed
    /// and replaced by parsing the `style` attribute and getting those
    /// properties from there.
    pub fn add_style_property(&mut self, name: &str, value: &str) {
        self.style_properties.insert(String::from(name), String::from(value));
    }

    /// Returns value of specified style property, if added to the node.
    pub fn get_style_property(&self, name: &str) -> Option<&String> {
        return self.style_properties.get(name);
    }

    /// Returns whether a style property with specified name was
    /// added to the node.
    pub fn has_style_property(&self, name: &str) -> bool {
        return self.style_properties.contains_key(name);
    }
}