// #![allow(dead_code)]
// use crate::error::*;
// use crate::expr::*;
// use crate::literal::*;

// struct AstPrinter;

// impl AstPrinter {
//     fn print(&self, expr: Box<dyn Expr<String>>) -> Result<String, LoxError> {
//         expr.accept(Box::new(self))
//     }

//     fn parenthesize(
//         &self,
//         name: &str,
//         exprs: &[&Box<dyn Expr<String>>],
//     ) -> Result<String, LoxError> {
//         let mut builder = String::new();
//         builder.push('(');
//         builder.push_str(name);
//         for expr in exprs {
//             builder.push(' ');
//             let s = expr.accept(Box::new(self))?;
//             builder.push_str(s.as_str());
//         }
//         builder.push(')');
//         Ok(builder)
//     }
// }

// impl Visitor<String> for AstPrinter {
//     fn visit_binary(&self, expr: &Binary<String>) -> Result<String, LoxError> {
//         let Binary {
//             left,
//             operator,
//             right,
//         } = expr;
//         self.parenthesize(&operator.lexeme, &[left, right])
//     }

//     fn visit_grouping(&self, expr: &Grouping<String>) -> Result<String, LoxError> {
//         let Grouping { expression } = expr;
//         self.parenthesize("group", &[expression])
//     }

//     fn visit_literal(&self, expr: &Literal) -> Result<String, LoxError> {
//         if expr.value.eq(&Object::Nil) {
//             return Ok(String::from("nil"));
//         }
//         Ok(expr.value.to_string())
//     }

//     fn visit_unary(&self, expr: &Unary<String>) -> Result<String, LoxError> {
//         let Unary { operator, right } = expr;
//         self.parenthesize(&operator.lexeme, &[right])
//     }
// }

// #[test]
// fn test_printer() {
//     use crate::literal::*;
//     use crate::token::*;

//     let expression: Binary<String> = Binary {
//         left: Box::new(Unary {
//             operator: Token::new(TokenType::Minus, String::from("-"), None, 1),
//             right: Box::new(Literal {
//                 value: Object::Num(123.),
//             }),
//         }),
//         operator: Token::new(TokenType::Star, String::from("*"), None, 1),
//         right: Box::new(Grouping {
//             expression: Box::new(Literal {
//                 value: Object::Num(45.67),
//             }),
//         }),
//     };
//     if let Ok(res) = AstPrinter.print(Box::new(expression)) {
//         println!("{}", res);
//     } else {
//         eprintln!("Failed to rloxed");
//     }
// }
