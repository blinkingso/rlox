//! Hand-write Expr file.

use crate::{error::LoxError, literal::Object, token::Token};

struct Binary<R> {
    left: Box<dyn Expr<R>>,
    operator: Token,
    right: Box<dyn Expr<R>>,
}

impl<R> Expr<R> for Binary<R> {
    fn accept(&self, visitor: Box<&dyn Visitor<R>>) -> Result<R, LoxError> {
        visitor.visit_binary(self)
    }
}

impl<R> Expr<R> for Unary<R> {
    fn accept(&self, visitor: Box<&dyn Visitor<R>>) -> Result<R, LoxError> {
        visitor.visit_unary(self)
    }
}

impl<R> Expr<R> for Grouping<R> {
    fn accept(&self, visitor: Box<&dyn Visitor<R>>) -> Result<R, LoxError> {
        visitor.visit_grouping(self)
    }
}

impl<R> Expr<R> for Literal {
    fn accept(&self, visitor: Box<&dyn Visitor<R>>) -> Result<R, LoxError> {
        visitor.visit_literal(self)
    }
}

struct Grouping<R> {
    expression: Box<dyn Expr<R>>,
}

struct Literal {
    value: Object,
}

struct Unary<R> {
    operator: Token,
    right: Box<dyn Expr<R>>,
}

trait Visitor<R> {
    fn visit_unary(&self, expr: &Unary<R>) -> Result<R, LoxError>;
    fn visit_binary(&self, expr: &Binary<R>) -> Result<R, LoxError>;
    fn visit_literal(&self, expr: &Literal) -> Result<R, LoxError>;
    fn visit_grouping(&self, expr: &Grouping<R>) -> Result<R, LoxError>;
}

trait Expr<R> {
    fn accept(&self, visitor: Box<&dyn Visitor<R>>) -> Result<R, LoxError>;
}

struct AstPrinter;
#[allow(dead_code)]
impl AstPrinter {
    fn print(&self, expr: Box<dyn Expr<String>>) -> Result<String, LoxError> {
        expr.accept(Box::new(self))
    }

    fn parenthesize(
        &self,
        name: &str,
        exprs: &[&Box<dyn Expr<String>>],
    ) -> Result<String, LoxError> {
        let mut builder = String::new();
        builder.push('(');
        builder.push_str(name);
        for expr in exprs {
            builder.push(' ');
            let s = expr.accept(Box::new(self))?;
            builder.push_str(s.as_str());
        }
        builder.push(')');
        Ok(builder)
    }
}

impl Visitor<String> for AstPrinter {
    fn visit_binary(&self, expr: &Binary<String>) -> Result<String, LoxError> {
        let Binary {
            left,
            operator,
            right,
        } = expr;
        self.parenthesize(&operator.lexeme, &[left, right])
    }

    fn visit_grouping(&self, expr: &Grouping<String>) -> Result<String, LoxError> {
        let Grouping { expression } = expr;
        self.parenthesize("group", &[expression])
    }

    fn visit_literal(&self, expr: &Literal) -> Result<String, LoxError> {
        if expr.value.eq(&Object::Nil) {
            return Ok(String::from("nil"));
        }
        Ok(expr.value.to_string())
    }

    fn visit_unary(&self, expr: &Unary<String>) -> Result<String, LoxError> {
        let Unary { operator, right } = expr;
        self.parenthesize(&operator.lexeme, &[right])
    }
}

#[test]
fn test_printer() {
    use crate::token::TokenType;

    let expression: Binary<String> = Binary {
        left: Box::new(Unary {
            operator: Token::new(TokenType::Minus, String::from("-"), None, 1),
            right: Box::new(Literal {
                value: Object::Num(123.),
            }),
        }),
        operator: Token::new(TokenType::Star, String::from("*"), None, 1),
        right: Box::new(Grouping {
            expression: Box::new(Literal {
                value: Object::Num(45.67),
            }),
        }),
    };
    if let Ok(res) = AstPrinter.print(Box::new(expression)) {
        println!("{}", res);
    } else {
        eprintln!("Failed to rloxed");
    }
}
