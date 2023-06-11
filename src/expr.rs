use crate::token::*;
use crate::literal::*;
 
pub enum Expr {
	Binary(BinaryExpr),
	Grouping(GroupingExpr),
	Literal(LiteralExpr),
	Unary(UnaryExpr),
}
pub struct BinaryExpr {
	pub left: Box<Expr>,
	pub operator: Token,
	pub right: Box<Expr>,
}

impl BinaryExpr {

pub fn accept<R>(self, visitor: Box<&dyn Visitor<R>>) -> ::std::io::Result<R> {
visitor.visit_binary_expr(Box::new(Expr::Binary(self)))
}

}
pub struct GroupingExpr {
	pub expression: Box<Expr>,
}

impl GroupingExpr {

pub fn accept<R>(self, visitor: Box<&dyn Visitor<R>>) -> ::std::io::Result<R> {
visitor.visit_grouping_expr(Box::new(Expr::Grouping(self)))
}

}
pub struct LiteralExpr {
	pub value: Object,
}

impl LiteralExpr {

pub fn accept<R>(self, visitor: Box<&dyn Visitor<R>>) -> ::std::io::Result<R> {
visitor.visit_literal_expr(Box::new(Expr::Literal(self)))
}

}
pub struct UnaryExpr {
	pub operator: Token,
	pub right: Box<Expr>,
}

impl UnaryExpr {

pub fn accept<R>(self, visitor: Box<&dyn Visitor<R>>) -> ::std::io::Result<R> {
visitor.visit_unary_expr(Box::new(Expr::Unary(self)))
}

}
pub trait Visitor<R> {
	fn visit_binary_expr(&self, expr: Box<Expr>) -> ::std::io::Result<R>;
	fn visit_grouping_expr(&self, expr: Box<Expr>) -> ::std::io::Result<R>;
	fn visit_literal_expr(&self, expr: Box<Expr>) -> ::std::io::Result<R>;
	fn visit_unary_expr(&self, expr: Box<Expr>) -> ::std::io::Result<R>;
}
