use crate::{expr::*, literal::Object};

struct AstPrinter;

impl Visitor<String> for AstPrinter {
    fn visit_binary_expr(&self, expr: Box<BinaryExpr>) -> std::io::Result<String> {
        let name = expr.operator.lexeme.to_string();
        self.parenthesize(name.as_str(), vec![*expr.left, *expr.right])
    }

    fn visit_grouping_expr(&self, expr: Box<GroupingExpr>) -> std::io::Result<String> {
        self.parenthesize("group", vec![*expr.expression])
    }

    fn visit_literal_expr(&self, expr: Box<LiteralExpr>) -> std::io::Result<String> {
        if *(expr.value) == Object::Nil {
            return Ok("nil".to_string());
        }
        Ok(expr.value.to_string())
    }

    fn visit_unary_expr(&self, expr: Box<UnaryExpr>) -> std::io::Result<String> {
        let name = expr.operator.lexeme.to_string();
        self.parenthesize(name.as_str(), vec![*expr.right])
    }
}

impl AstPrinter {
    fn print(&self, expr: Expr) -> std::io::Result<String> {
        match expr {
            Expr::Binary(b) => b.accept(Box::new(self)),
            Expr::Unary(b) => b.accept(Box::new(self)),
            Expr::Literal(b) => b.accept(Box::new(self)),
            Expr::Grouping(b) => b.accept(Box::new(self)),
        }
    }

    fn parenthesize(&self, name: &str, exprs: Vec<Expr>) -> std::io::Result<String> {
        let mut builder = String::new();
        builder.push_str("(");
        builder.push_str(name);
        for expr in exprs.into_iter() {
            builder.push_str(" ");
            let s: String = match expr {
                Expr::Binary(b) => b.accept(Box::new(self))?,
                Expr::Unary(b) => b.accept(Box::new(self))?,
                Expr::Literal(b) => b.accept(Box::new(self))?,
                Expr::Grouping(b) => b.accept(Box::new(self))?,
            };
            builder.push_str(s.as_str());
        }
        builder.push_str(")");
        Ok(builder)
    }
}

#[test]
fn test_printer() {
    use crate::token::*;

    let expression = BinaryExpr {
        left: Box::new(Expr::Unary(Box::new(UnaryExpr {
            operator: Box::new(Token::new(TokenType::Minus, "-".to_string(), None, 1)),
            right: Box::new(Expr::Literal(Box::new(LiteralExpr {
                value: Box::new(Object::Num(123.)),
            }))),
        }))),
        right: Box::new(Expr::Grouping(Box::new(GroupingExpr {
            expression: Box::new(Expr::Literal(Box::new(LiteralExpr {
                value: Box::new(Object::Num(45.67)),
            }))),
        }))),
        operator: Box::new(Token::new(TokenType::Star, "*".to_string(), None, 1)),
    };

    let ast = AstPrinter;
    match ast.print(Expr::Binary(Box::new(expression))) {
        Ok(s) => println!("{s}"),
        Err(e) => eprintln!("failed to parse expression for : {:?}", e),
    }
}
