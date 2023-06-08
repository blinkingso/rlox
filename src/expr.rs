use crate::literal::*;
use crate::token::*;

pub enum Expr {
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Unary(UnaryExpr),
}
pub struct BinaryExpr {
    left: Box<Expr>,
    operator: Token,
    right: Box<Expr>,
}
pub struct GroupingExpr {
    expression: Box<Expr>,
}
pub struct LiteralExpr {
    value: Object,
}
pub struct UnaryExpr {
    operator: Token,
    right: Box<Expr>,
}
trait Visitor<R> {
    fn visit_binary_expr(expr: Box<BinaryExpr>) -> ::std::io::Result<R>;
    fn visit_grouping_expr(expr: Box<GroupingExpr>) -> ::std::io::Result<R>;
    fn visit_literal_expr(expr: Box<LiteralExpr>) -> ::std::io::Result<R>;
    fn visit_unary_expr(expr: Box<UnaryExpr>) -> ::std::io::Result<R>;
}
