use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse_macro_input;

use crate::expr::ExprAst;

mod expr;

#[proc_macro]
pub fn define_ast(input: TokenStream) -> TokenStream {
    let mut output = quote!();

    let expr_ast = parse_macro_input!(input as ExprAst);
    let base_ident = &expr_ast.ident;
    let structs = &expr_ast.structs;

    // define `Visitor` trait
    let visitor_generic_method_names = structs
        .iter()
        .filter(|s| s.is_generic())
        .map(|s| (format_ident!("visit_{}", s.get_ident_name_lowercase())))
        .collect::<Vec<_>>();
    let visitor_generic_idents = structs
        .iter()
        .filter(|s| s.is_generic())
        .map(|s| s.ident.clone())
        .collect::<Vec<_>>();
    let visitor_none_generic_method_names = structs
        .iter()
        .filter(|s| !s.is_generic())
        .map(|s| (format_ident!("visit_{}", s.get_ident_name_lowercase())))
        .collect::<Vec<_>>();
    let visitor_none_generic_idents = structs
        .iter()
        .filter(|s| !s.is_generic())
        .map(|s| s.ident.clone())
        .collect::<Vec<_>>();

    output.extend(quote! {
        pub trait Visitor<R> {
            #(
                fn #visitor_generic_method_names(&self, expr: &#visitor_generic_idents<R>) -> Result<R, LoxError>;
            )*

            #(
                fn #visitor_none_generic_method_names(&self, expr: &#visitor_none_generic_idents) -> Result<R, LoxError>;
            )*
        }
    });

    // define `Expr` trait
    output.extend(quote! {
        pub trait #base_ident<R> {
            fn accept(&self, visitor: Box<&dyn Visitor<R>>) -> Result<R, LoxError>;
        }
    });

    // define structs.
    structs.iter().for_each(|s| {
        let ident = &s.ident;
        let visitor_name = format_ident!("visit_{}", s.get_ident_name_lowercase());
        let generic_field_names = s
            .fields
            .iter()
            .filter(|f| f.is_generic())
            .map(|f| f.name.clone())
            .collect::<Vec<_>>();
        let generic_field_tys = s
            .fields
            .iter()
            .filter(|f| f.is_generic())
            .map(|f| f.ty.clone())
            .collect::<Vec<_>>();
        let none_generic_field_names = s
            .fields
            .iter()
            .filter(|f| !f.is_generic())
            .map(|f| f.name.clone())
            .collect::<Vec<_>>();
        let none_generic_field_tys = s
            .fields
            .iter()
            .filter(|f| !f.is_generic())
            .map(|f| f.ty.clone())
            .collect::<Vec<_>>();

        if s.is_generic() {
            output.extend(quote! {
                    pub struct #ident<R> {
                        #(
                            pub #generic_field_names: Box<dyn #generic_field_tys<R>>,
                        )*

                        #(
                            pub #none_generic_field_names: #none_generic_field_tys,
                        )*
                    }

                    impl<R> #base_ident<R> for #ident<R> {
                        fn accept(&self, visitor: Box<&dyn Visitor<R>>) -> Result<R, LoxError> {
                            visitor.#visitor_name(self)
                        }
                    }
            });
        } else {
            output.extend(quote! {
                pub struct #ident {
                    #(
                        pub #none_generic_field_names: #none_generic_field_tys,
                    )*
                }

                impl<R> #base_ident<R> for #ident {
                    fn accept(&self, visitor: Box<&dyn Visitor<R>>) -> Result<R, LoxError> {
                        visitor.#visitor_name(self)
                    }
                }
            });
        }
    });
    output.into()
}
