#[macro_export]
macro_rules! define_ast {
    (
        $base:ident,
        [(
            $struct_name:ty,
            (
                $field_ty:ty, $field_name:ident,
            )*
        )*]
    ) => {
        struct $struct_name<R> {
            $($field_name: $field_ty,)*
        }
    };
}

use crate::token::Token;

define_ast!(Expr, [(Binary, (Expr, left, Token, operator)),]);
