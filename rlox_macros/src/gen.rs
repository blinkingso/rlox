use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse::Parse, parse_macro_input, spanned::Spanned, token::Comma, Expr, ExprArray, ExprLit,
    Generics, Lit, Type,
};

#[derive(Debug)]
struct ExprItem {
    base_name: Ident,
    types: Vec<ClassItem>,
}

#[derive(Debug)]
struct ClassItem {
    class: Ident,
    fields: Vec<FieldItem>,
}

#[derive(Debug)]
struct FieldItem {
    field_name: Ident,
    field_ty: Ident,
}

/// Define ast.
///
/// ```rust
/// define_ast(
///     Expr,
///     [
///         "Binary : Expr left, Token operator, Expr right",
///         "Grouping : Expr expression",
///         "Literal : Object value",
///         "Unary : Token operator, Expr right"
///     ]
/// )
/// ```
impl Parse for ExprItem {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // 1. parse base_name `Expr`
        let base_name: Ident = input.parse()?;

        // 2. parse a comma
        let _ = input.parse::<Comma>()?;

        // 3. parse types
        let classes: ExprArray = input.parse()?;
        let class_items = classes
            .elems
            .iter()
            .map(|ele| {
                if let Expr::Lit(lit) = ele {
                    if let Lit::Str(ref string) = lit.lit {
                        let class_string = string.value();
                        let class_string = class_string.trim();
                        let (struct_name, fields) = class_string.split_once(":").unwrap();
                        let struct_name = struct_name.trim();
                        let fields = fields.trim();
                        let fields = fields.split(",");
                        let field_items = fields
                            .into_iter()
                            .map(|f| f.trim())
                            .map(|field| {
                                let (field_ty, field_name) = field.split_once(" ").unwrap();
                                let field_item = FieldItem {
                                    field_name: Ident::new(field_name, Span::call_site()),
                                    field_ty: Ident::new(field_ty, Span::call_site()),
                                };
                                field_item
                            })
                            .collect::<Vec<FieldItem>>();
                        ClassItem {
                            class: Ident::new(struct_name, Span::call_site()),
                            fields: field_items,
                        }
                    } else {
                        panic!("Illegal literal.");
                    }
                } else {
                    panic!("Illegal literal.");
                }
            })
            .collect();
        Ok(ExprItem {
            base_name,
            types: class_items,
        })
    }
}

pub fn gen_ast(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as ExprItem);
    let base = item.base_name;
    let generics = Generics::default();
    // 0. define base trait
    let mut output = quote!(
        use crate::token::*;
        use crate::literal::*;
        use crate::error::*;

        pub trait Visitor<R> {
            fn visit_unary(&self, expr: &Unary<R>) -> Result<R, LoxError>;
            fn visit_binary(&self, expr: &Binary<R>) -> Result<R, LoxError>;
            fn visit_literal(&self, expr: &Literal) -> Result<R, LoxError>;
            fn visit_grouping(&self, expr: &Grouping<R>) -> Result<R, LoxError>;
        }

        pub trait #base #generics {
            fn accept(&self, visitor: Box<&dyn Visitor #generics>) -> Result<#generics, LoxError>;
        }
    );

    // 1. define structs
    for class in item.types.iter() {
        let class_ty = &class.class;
        let fields: Vec<Ident> = class
            .fields
            .iter()
            .map(|f| {
                let fname = &f.field_name;
                let fty = &f.field_ty;
                if base.eq(fty) {
                    format_ident!("{}: Box<dyn {}>", fname, fty)
                } else {
                    format_ident!("{}: {}", fname, fty)
                }
            })
            .collect();
        output.extend(quote!(
            struct #class_ty #generics {
                #(#fields),*
            }
        ));
    }
    output.into()
}

#[test]
fn test_items() {
    let s = quote!(
        Expr,
        [
            "Binary : Expr left, Token operator, Expr right",
            "Grouping : Expr expression",
            "Literal : Object value",
            "Unary : Token operator, Expr right"
        ]
    );
    let s: ExprItem = syn::parse2(s.into()).unwrap();
    println!("s: {:#?}", s);
}
