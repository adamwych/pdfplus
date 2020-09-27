mod parser;
mod document;

pub use self::parser::parse_text;
pub use self::document::{
    Document,
    DocumentRef,
    Element
};