use crate::{
    expr::*,
    literal::Object,
    token::{Token, TokenType},
};

struct AstPrinter;
impl Visitor<String> for AstPrinter {
    fn visit_binary_expr(&self, expr: Box<Expr>) -> std::io::Result<String> {
        if let Expr::Binary(ref binary) = *expr {
            let name = binary.operator.lexeme.clone();
            self.parenthesize(name.as_str(), vec![expr])
        } else {
            Err(std::io::Error::from_raw_os_error(1))
        }
    }
    fn visit_grouping_expr(&self, expr: Box<Expr>) -> std::io::Result<String> {
        if let Expr::Grouping(grouping) = *expr {
            self.parenthesize("group", vec![grouping.expression])
        } else {
            Err(std::io::Error::from_raw_os_error(1))
        }
    }

    fn visit_literal_expr(&self, expr: Box<Expr>) -> std::io::Result<String> {
        if let Expr::Literal(ref literal) = *expr {
            if literal.value == Object::Nil {
                return Ok("nil".to_string());
            }
            Ok(literal.value.to_string())
        } else {
            Err(std::io::Error::from_raw_os_error(1))
        }
    }
    fn visit_unary_expr(&self, expr: Box<Expr>) -> std::io::Result<String> {
        if let Expr::Unary(ref unary) = *expr {
            let name = unary.operator.lexeme.clone();
            self.parenthesize(name.as_str(), vec![expr])
        } else {
            Err(std::io::Error::from_raw_os_error(1))
        }
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

    fn parenthesize(&self, name: &str, exprs: Vec<Box<Expr>>) -> std::io::Result<String> {
        let mut builder = String::new();
        builder.push_str("(");
        builder.push_str(name);
        for expr in exprs.into_iter() {
            builder.push_str(" ");
            let s: String = match *expr {
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

// #[test]
// fn test_printer() {
//     std::env::set_var("RLOX_AST_DIR", "src");
//     let expression = BinaryExpr {
//         left: Box::new(Expr::Unary(UnaryExpr {
//             operator: Token::new(TokenType::Minus, "-".to_string(), None, 1),
//             right: Box::new(Expr::Literal(LiteralExpr {
//                 value: Object::Num(123.),
//             })),
//         })),
//         right: Box::new(Expr::Grouping(GroupingExpr {
//             expression: Box::new(Expr::Literal(LiteralExpr {
//                 value: Object::Num(45.67),
//             })),
//         })),
//         operator: Token::new(TokenType::Star, "*".to_string(), None, 1),
//     };

//     let ast = AstPrinter;
//     match ast.print(Expr::Binary(expression)) {
//         Ok(s) => println!("{s}"),
//         Err(e) => eprintln!("failed to parse expression for : {:?}", e),
//     }
// }
