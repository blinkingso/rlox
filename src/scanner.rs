use crate::{
    error::LoxError,
    literal::Object,
    token::{Token, TokenType},
};

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) -> Result<(), LoxError> {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                let token_type = if self.is_match('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(token_type)
            }
            '=' => {
                let token_type = if self.is_match('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(token_type)
            }
            '<' => {
                let token_type = if self.is_match('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(token_type)
            }
            '>' => {
                let token_type = if self.is_match('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(token_type)
            }
            '/' => {
                if self.is_match('/') {
                    // A comment goes until the end of the line.
                    while let Some(ch) = self.peek() {
                        if ch != '\n' {
                            self.advance();
                        }
                    }
                } else if self.is_match('*') {
                    // comments
                    self.scan_comment()?;
                } else {
                    self.add_token(TokenType::Slash)
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => self.string()?,

            // Reserved Words and Identifiers.
            'o' => {
                if self.is_match('r') {
                    self.add_token(TokenType::Or);
                }
            }
            ch => {
                if Scanner::is_digit(ch) {
                    self.number()?;
                } else if Scanner::is_alpha(ch) {
                    self.identifier();
                } else {
                    return Err(LoxError::error(
                        self.line,
                        format!("Unexpected character: {}", ch),
                    ));
                }
            }
        }
        Ok(())
    }

    fn scan_comment(&mut self) -> Result<(), LoxError> {
        loop {
            match self.peek() {
                Some('*') => {
                    self.advance();
                    if self.is_match('/') {
                        if self.is_match('*') {
                            self.scan_comment()?;
                        } else {
                            return Ok(());
                        }
                    }
                }
                Some('/') => {
                    self.advance();
                    if self.is_match('*') {
                        self.scan_comment()?;
                    }
                }
                Some('\n') => {
                    self.advance();
                    self.line += 1;
                }
                Some(_ch) => {
                    self.advance();
                }
                None => {
                    // at the end of the file.
                    return Err(LoxError::error(
                        self.line,
                        "Unterminated comments".to_string(),
                    ));
                }
            }
        }
    }

    fn identifier(&mut self) {
        while let Some(ch) = self.peek() {
            if Scanner::is_alpha_numberic(ch) {
                self.advance();
            } else {
                break;
            }
        }
        let text: String = String::from_iter(self.source[self.start..self.current].into_iter());
        let tt = match text.as_str() {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "true" => TokenType::True,
            "for" => TokenType::For,
            "if" => TokenType::If,
            "fun" => TokenType::Fun,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => {
                self.add_token(TokenType::Identifier);
                return;
            }
        };
        self.add_token(tt);
    }

    fn is_alpha(c: char) -> bool {
        c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z' || c == '_'
    }

    fn is_alpha_numberic(c: char) -> bool {
        Scanner::is_digit(c) || Scanner::is_alpha(c)
    }

    fn string(&mut self) -> Result<(), LoxError> {
        while let Some(ch) = self.peek() {
            if !self.is_at_end() && ch != '"' {
                if ch == '\n' {
                    self.line += 1;
                }
                self.advance();
            } else {
                break;
            }
        }

        if self.is_at_end() {
            return Err(LoxError::error(
                self.line,
                "Unterminated string.".to_string(),
            ));
        }

        // The closing '"'.
        self.advance();

        let value = String::from_iter(self.source[self.start + 1..self.current - 1].into_iter());
        self.add_token_string(TokenType::String, Some(Object::String(value)));

        Ok(())
    }

    fn peek(&mut self) -> Option<char> {
        if self.is_at_end() {
            return None;
        }
        // char_at(self.source.as_str(), self.current)
        Some(*self.source.get(self.current).unwrap())
    }

    fn is_match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        match self.source.get(self.current) {
            Some(v) if *v == expected => {
                self.current += 1;
                true
            }
            _ => false,
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_string(token_type, None)
    }

    fn add_token_string(&mut self, token_type: TokenType, literal: Option<Object>) {
        let lexeme = String::from_iter(self.source[self.start..self.current].iter());
        self.tokens
            .push(Token::new(token_type, lexeme, literal, self.line));
    }

    fn advance(&mut self) -> char {
        let ch = *self.source.get(self.current).unwrap();
        self.current += 1;
        ch
    }
    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, LoxError> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        self.tokens.push(Token::eof(self.line));
        Ok(&self.tokens)
    }

    pub fn new(source: String) -> Scanner {
        Scanner {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn is_digit(c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn number(&mut self) -> Result<(), LoxError> {
        while let Some(ch) = self.peek() {
            if Scanner::is_digit(ch) {
                self.advance();
            } else {
                break;
            }
        }

        // Look for a fractional part.
        if self.is_match('.') {
            while let Some(next) = self.peek_next() {
                if Scanner::is_digit(next) {
                    // Consume the '.'
                    self.advance();
                } else {
                    break;
                }
            }
        }

        let num_string = String::from_iter(self.source[self.start..self.current].iter());
        self.add_token_string(
            TokenType::Number,
            Some(Object::Num(num_string.parse().unwrap())),
        );
        Ok(())
    }

    fn peek_next(&mut self) -> Option<char> {
        if self.current + 1 >= self.source.len() {
            return None;
        }
        Some(*self.source.get(self.current + 1).unwrap())
    }
}
