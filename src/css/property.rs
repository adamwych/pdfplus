use crate::css::primitive_value::{PrimitiveValue};

#[derive(Debug)]
pub struct PropertyDeclaration {
    pub name: String,
    pub value: PrimitiveValue,
}

impl PropertyDeclaration {
    pub fn new(name: String) -> PropertyDeclaration {
        PropertyDeclaration {
            name: name,
            value: PrimitiveValue::default()
        }
    }
}