#![allow(dead_code)]

use std::ops::IndexMut;
use std::rc::Rc;
use std::cell::RefCell;
use crate::css::PrimitiveValue;
use crate::html::{Element, ElementStyleProperties};

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

    /// Applies element's styling properties to all its descendants.
    /// Not all properties will be applied - only those, which are
    /// supposed to be shared between parents and their children (e.g. text color).
    pub fn cascade_element_styles(&mut self, element_index: usize) {
        let mut doc = self.clone();
        let element = self.elements.index_mut(element_index);
        let parent_props = element.get_style_properties();
        let children_num = element.children.len();

        for child_index_i in 0..children_num {
            let child_index = element.children[child_index_i];
            let child = doc.get_element(child_index);
            child.set_style_properties(ElementStyleProperties::merge(child.get_style_properties(), parent_props));
            doc.cascade_element_styles(child_index);
        }

        self.elements = doc.elements;
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
        let mut element = Element::default();
        element.index = idx;
        element.tag = String::from(tag);
        element.add_default_style_properties();

        self.elements.push(element);
        return idx;
    }

    /// Creates a new text Element and returns its index.
    pub fn create_text_element(&mut self, text: &str) -> usize {
        let idx = self.elements.len();
        let mut element = Element::default();
        element.index = idx;
        element.tag = String::from("#text");
        element.text = String::from(text);
        element.add_default_style_properties();

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
