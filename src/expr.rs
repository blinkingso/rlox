use crate::error::LoxError;
use crate::literal::Object;
use crate::token::Token;

rlox_macros::define_ast! {
    "Expr",
    [
        "Binary   : Expr left, Token operator, Expr right",
        "Grouping : Expr expression",
        "Literal  : Object value",
        "Unary    : Token operator, Expr right"
    ]
}
