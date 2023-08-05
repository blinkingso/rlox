use crate::expr;
use crate::expr::*;
use crate::literal::Object;
use crate::token::Token;
use crate::token::TokenType;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    fn expression<R>(&self) -> Box<dyn Expr<R>> {
        self.equality()
    }

    fn equality<R>(&self) -> Box<dyn Expr<R>> {
        let mut expr = self.comparison();

        while self.match_token_type(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison();
            expr = Box::new(Binary {
                expr,
                operator,
                right,
            });
        }
        expr
    }

    fn match_token_type(&mut self, types: Vec<TokenType>) -> bool {
        for ty in types.into_iter() {
            if self.check(ty) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn check(&self, ty: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().token_type == ty
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> Token {
        self.tokens.get(self.current).unwrap().clone()
    }

    fn previous(&self) -> Token {
        self.tokens.get(self.current - 1).unwrap().clone()
    }

    fn comparison<R>(&self) -> Box<dyn Expr<R>> {
        let mut expr = self.term();
        while self.match_token_type(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term();
            expr = Box::new(Binary {
                expr,
                operator,
                right,
            });
        }
        expr
    }

    fn term<R>(&self) -> Box<dyn Expr<R>> {
        let mut expr = self.factor();
        while self.match_token_type(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor();
            expr = Box::new(Binary {
                expr,
                operator,
                right,
            });
        }
        expr
    }

    fn factor<R>(&self) -> Box<dyn Expr<R>> {
        let mut expr = self.unary();

        while self.match_token_type(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary();
            expr = Box::new(Binary {
                expr,
                operator,
                right,
            })
        }
        expr
    }

    fn unary<R>(&self) -> Box<dyn Expr<R>> {
        if self.match_token_type(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary();
            return Box::new(Unary { operator, right });
        }
        self.primary()
    }

    fn primary<R>(&self) -> Box<dyn Expr<R>> {
        if self.match_token_type(vec![TokenType::False]) {
            return Box::new(Literal {
                value: Object::False,
            });
        }
        if self.match_token_type(vec![TokenType::True]) {
            return Box::new(Literal {
                value: Object::True,
            });
        }
        if self.match_token_type(vec![TokenType::Nil]) {
            return Box::new(Literal { value: Object::Nil });
        }
        if self.match_token_type(vec![TokenType::Number, TokenType::String]) {
            return Box::new(Literal {
                value: self.previous().literal.unwrap(),
            });
        }
        if self.match_token_type(vec![TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expect ')' after expression.");
            return Box::new(Grouping { expression: expr });
        }
    }
}
