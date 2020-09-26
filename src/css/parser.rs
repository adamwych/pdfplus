use crate::color;
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

        match property_name.as_str() {
            "color" | "background-color" => {
                declaration.value = self.parse_color_value();
                return Some(declaration);
            }

            _ => {
                println!("unsupported property declaration: {}", property_name);
                self.buffer.skip_until(TokenKind::Semicolon);
                self.buffer.expect(TokenKind::Semicolon);
                return None;
            }
        }
    }

    fn parse_color_value(&mut self) -> PrimitiveValue {
        let token = self.buffer.next();
        let value = &token.value;

        match token.kind {
            TokenKind::Hash |
            TokenKind::Identifier => {
                return PrimitiveValue::from_color(&value, color::code_to_color(&value));
            }

            _ => {
                return PrimitiveValue::default();
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