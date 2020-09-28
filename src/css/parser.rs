use crate::utils;
use crate::css::PropertyDeclaration;
use crate::css::PrimitiveValue;
use crate::css::ParserTokenBuffer;
use crate::css::tokenizer::{tokenize, TokenKind};

pub struct Parser {
    input: String,
    buffer: ParserTokenBuffer
}

impl Parser {
    pub fn parse_inline(&mut self) -> Vec<PropertyDeclaration> {
        let mut result = Vec::<PropertyDeclaration>::new();
        self.buffer = ParserTokenBuffer::new(tokenize(self.input.as_str()));

        while !self.buffer.is_out_of_bounds() {
            let token = self.buffer.next();
            if token.kind == TokenKind::EndOfInput {
                break;
            }

            if token.kind == TokenKind::Identifier {
                if let Some(declaration) = self.parse_declaration() {
                    result.push(declaration);
                }
            }
        }

        return result;
    }

    fn parse_declaration(&mut self) -> Option<PropertyDeclaration> {
        let property_name = self.buffer.current().value.clone();
        let mut declaration = PropertyDeclaration::new(property_name.clone());
        self.buffer.expect(TokenKind::Colon);
        let mut value: Option<PrimitiveValue> = None;
        let mut supported = true;

        match property_name.as_str() {
            "width" | "height" => {
                value = self.parse_generic_dimension_value();
            }

            "display" => {
                value = self.parse_display_value();
            }

            "color" | "background-color" => {
                value = self.parse_color_value();
            }

            _ => {
                supported = false;
                println!("unsupported property declaration: {}", property_name);
                self.buffer.skip_until(TokenKind::Semicolon);
                self.buffer.expect(TokenKind::Semicolon);
            }
        }

        if supported {
            if value.is_none() {
                println!("invalid '{}' property value - cannot be {}", property_name, self.buffer.current().kind.to_string());
                return None;
            }
    
            declaration.value = value.unwrap();
        }

        return Some(declaration);
    }

    fn parse_generic_dimension_value(&mut self) -> Option<PrimitiveValue> {
        let token = self.buffer.next();
        match token.kind {
            TokenKind::Dimension => {
                return Some(PrimitiveValue::from_dimension_value(&token.value, token.value.to_string().parse().unwrap(), &token.unit));
            }

            _ => {
                return None;
            }
        }
    }

    fn parse_display_value(&mut self) -> Option<PrimitiveValue> {
        let token = self.buffer.next();
        match token.kind {
            TokenKind::Identifier => {
                return Some(PrimitiveValue::from_identifier(&token.value));
            }

            _ => {
                return None;
            }
        }
    }

    fn parse_color_value(&mut self) -> Option<PrimitiveValue> {
        let token = self.buffer.next();
        match token.kind {
            TokenKind::Hash |
            TokenKind::Identifier => {
                return Some(PrimitiveValue::from_color(&token.value, utils::color::code_to_color(&token.value)));
            }

            _ => {
                return None;
            }
        }
    }

    pub fn new(input: String) -> Parser {
        Parser {
            input: input,
            buffer: ParserTokenBuffer::default()
        }
    }
}

pub fn parse_inline(input: &str) -> Vec<PropertyDeclaration> {
    return Parser::new(input.to_string()).parse_inline();
}