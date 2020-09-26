#![allow(unused)]

// This tokenizer tries to be pretty conformant with the specification here:
// https://www.w3.org/TR/css-syntax-3/#tokenization but full conformance is not the final goal.
//
// Currently only supports tokenizing simple, inline inputs (no selectors etc.).

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Identifier,
    Function,
    At,
    Hash,
    String,
    Url,
    Delimeter,
    Integer,
    Number,
    Percentage,
    Dimension,
    CDO,
    CDC,
    Colon,
    Semicolon,
    Comma,
    SquareParenthesisOpen,
    SquareParenthesisClose,
    ParenthesisOpen,
    ParenthesisClose,
    BracketOpen,
    BracketClose,
    EndOfInput
}

impl ToString for TokenKind {
    fn to_string(&self) -> String {
        let val = match self {
            TokenKind::Identifier => "identifier",
            TokenKind::Function => "function",
            TokenKind::At => "@",
            TokenKind::Hash => "#",
            TokenKind::String => "string",
            TokenKind::Url => "url",
            TokenKind::Delimeter => "delimeter",
            TokenKind::Integer => "integer",
            TokenKind::Number => "number",
            TokenKind::Percentage => "%",
            TokenKind::Dimension => "dimension",
            TokenKind::CDO => "<!--",
            TokenKind::CDC => "-->",
            TokenKind::Colon => ":",
            TokenKind::Semicolon => ";",
            TokenKind::Comma => ",",
            TokenKind::SquareParenthesisOpen => "[",
            TokenKind::SquareParenthesisClose => "]",
            TokenKind::ParenthesisOpen => "(",
            TokenKind::ParenthesisClose => ")",
            TokenKind::BracketOpen => "{",
            TokenKind::BracketClose => "}",
            TokenKind::EndOfInput => "end of input",
            _ => ""
        };

        return val.to_string();
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
    pub unit: String,
}

impl Token {
    pub fn is_int(&self) -> bool {
        return self.value.contains('.');
    }

    pub fn simple(kind: TokenKind) -> Token {
        Token {
            kind: kind,
            value: String::new(),
            unit: String::new()
        }
    }

    pub fn for_char(kind: TokenKind, value: char) -> Token {
        Token {
            kind: kind,
            value: String::from(value),
            unit: String::new()
        }
    }

    pub fn for_string(kind: TokenKind, value: String) -> Token {
        Token {
            kind: kind,
            value: value,
            unit: String::new()
        }
    }

    pub fn for_dimension(value: String, unit: String) -> Token {
        Token {
            kind: TokenKind::Dimension,
            value: value,
            unit: unit
        }
    }
}

pub struct Tokenizer {
    input: Vec<u8>,
    buffer_pos: usize,
    buffer_len: usize
}

impl Tokenizer {
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::<Token>::new();

        loop {
            let token = self.next();
            let is_end = token.kind == TokenKind::EndOfInput;
            tokens.push(token);
            if is_end {
                break;
            }
        }

        return tokens;
    }

    fn next(&mut self) -> Token {
        if self.is_out_of_bounds() {
            return Token::simple(TokenKind::EndOfInput);
        }

        let ch = self.next_char();

        if ch.is_whitespace() {
            return self.next();
        }

        // <ident-token>
        if ch.is_alphabetic() || ch == '-' {
            if !self.peek_char().is_numeric() {
                let value = self.read_identifier();
    
                // might become <function-token>
                if self.peek_char() == '(' {
                    self.next_char();
                    return Token::for_string(TokenKind::Function, value);
                }
    
                return Token::for_string(TokenKind::Identifier, value);
            }
        }

        if ch.is_numeric() || ch == '+' || ch == '-' {
            let (value, is_int) = self.read_number();

            if self.would_start_identifier() {
                self.next_char();
                let dimension = self.read_identifier();
                return Token::for_dimension(value, dimension);
            }

            if self.peek_char() == '%' {
                self.next_char();
                return Token::for_string(TokenKind::Percentage, value);
            }

            if is_int {
                return Token::for_string(TokenKind::Integer, value);
            } else {
                return Token::for_string(TokenKind::Number, value);
            }
        }

        // <string-token>
        if ch == '"' || ch == '\'' {
            self.next_char();
            let value = self.read_string(ch);
            return Token::for_string(TokenKind::String, value);
        }

        if ch == '@' {
            let value = self.read_identifier();
            return Token::for_string(TokenKind::At, value);
        }

        if ch == '#' {
            self.next_char();
            let value = self.read_identifier();
            return Token::for_string(TokenKind::Hash, value);
        }

        if ch == '/' && self.peek_char() == '*' {
            while !self.is_out_of_bounds() {
                let ch = self.next_char();
                if ch == '*' && self.peek_char() == '/' {
                    self.next_char();
                    break;
                }
            }

            return self.next();
        }

        // other possibilities
        match ch {
            ',' => Token::for_char(TokenKind::Comma, ch),
            ':' => Token::for_char(TokenKind::Colon, ch),
            ';' => Token::for_char(TokenKind::Semicolon, ch),
            _ => Token::for_char(TokenKind::Delimeter, ch)
        }
    }

    fn would_start_identifier(&mut self) -> bool {
        let x1 = self.peek_char();
        if x1 == '-' {
            let x2 = self.next_char();
            if x2.is_alphanumeric() || x2 == '-' {
                self.previous_char();
                return true;
            }

            self.previous_char();
        } else {
            if x1.is_alphanumeric() {
                return true;
            }
        }

        return false;
    }

    fn current_char(&mut self) -> char {
        return self.input[self.buffer_pos - 1] as char;
    }

    fn peek_char(&mut self) -> char {
        if self.is_out_of_bounds() {
            return '\0';
        }

        return self.input[self.buffer_pos] as char;
    }

    fn next_char(&mut self) -> char {
        let c = self.input[self.buffer_pos] as char;
        self.buffer_pos += 1;
        return c;
    }

    fn previous_char(&mut self) {
        self.buffer_pos -= 1;
    }

    fn read_string(&mut self, end_char: char) -> String {
        let mut value = String::from(self.current_char());

        while !self.is_out_of_bounds() {
            let ch = self.next_char();

            if self.is_newline(ch) || ch == end_char {
                break;
            }

            value.push(ch);
        }

        return value;
    }

    fn read_number(&mut self) -> (String, bool) {
        let mut value = String::from(self.current_char());
        let mut has_dot = false;
        let mut is_int = true;

        while !self.is_out_of_bounds() {
            let ch = self.next_char();

            if ch == '.' {
                if has_dot {
                    self.previous_char();
                    break;
                } else {
                    is_int = false;
                    has_dot = true;
                }
            } else {
                if !ch.is_numeric() {
                    self.previous_char();
                    break;
                }
            }

            value.push(ch);
        }

        return (value, is_int);
    }
    
    fn read_identifier(&mut self) -> String {
        // skip up to 2 dashes at the beginning
        if self.current_char() == '-' {
            self.next_char();

            if self.current_char() == '-' {
                self.next_char();
            }
        }

        let mut identifier = String::from(self.current_char());

        while !self.is_out_of_bounds() {
            let ch = self.next_char();

            if !self.is_allowed_identifier_char(ch) {
                self.previous_char();
                break;
            }

            identifier.push(ch);
        }

        return identifier;
    }

    fn is_newline(&self, ch: char) -> bool {
        // specification also uses \f but it's pretty old so skipping
        // it should not make any problems
        return ch == '\r' || ch == '\n';
    }

    fn is_out_of_bounds(&self) -> bool {
        return self.buffer_pos >= self.buffer_len;
    }

    fn is_allowed_identifier_char(&self, c: char) -> bool {
        return c.is_alphanumeric() || c == '_' || c == '-';
    }

    pub fn new(input: &str) -> Self {
        Self {
            input: Vec::from(input.as_bytes()),
            buffer_pos: 0,
            buffer_len: input.len()
        }
    }
}

pub fn tokenize(input: &str) -> Vec<Token> {
    return Tokenizer::new(input).tokenize();
}