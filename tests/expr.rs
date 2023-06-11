use rlox_derive_lib::define_ast;

#[test]
fn test_expr() {
    define_ast! {
        "src",
        Expr,
        [
        "Binary   : Expr left, Token operator",
        "Expr right", "Grouping : Expr expression",
        "Literal  : Object value",
        "Unary    : Token operator, Expr right"
        ]
    }
}
