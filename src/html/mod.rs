mod parser;
mod document;
mod element;
mod element_style_properties;

pub use self::parser::parse_text;
pub use self::document::{
    Document,
    DocumentRef,
};

pub use self::element::Element;
pub use self::element_style_properties::ElementStyleProperties;