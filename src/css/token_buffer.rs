use crate::css::tokenizer::{Token, TokenKind};

pub struct ParserTokenBuffer {
    pos: usize,
    len: usize,
    tokens: Vec<Token>
}

impl ParserTokenBuffer {
    pub fn next(&mut self) -> &Token {
        let token = &self.tokens[self.pos];
        self.pos += 1;
        return &token;
    }

    pub fn skip_until(&mut self, kind: TokenKind) {
        while !self.is_out_of_bounds() {
            let token = self.next();
            if token.kind == kind {
                self.pos -= 1;
                break;
            }
        }
    }

    pub fn expect(&mut self, kind: TokenKind) -> &Token {
        let tok = &self.tokens[self.pos];
        if tok.kind != kind {
            println!("syntax error: unexpected token '{}', expected '{}'", tok.kind.to_string(), kind.to_string());
        }

        self.pos += 1;

        return tok;
    }

    pub fn current(&self) -> &Token {
        return &self.tokens[self.pos - 1];
    }

    pub fn is_out_of_bounds(&self) -> bool {
        return self.pos >= self.len;
    }

    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            pos: 0,
            len: tokens.len(),
            tokens: tokens
        }
    }

    pub fn default() -> Self {
        Self {
            pos: 0,
            len: 0,
            tokens: Vec::new()
        }
    }
}