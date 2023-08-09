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
    let visit_method_names = structs
        .iter()
        .map(|s| (format_ident!("visit_{}", s.get_ident_name_lowercase())))
        .collect::<Vec<_>>();
    let visit_idents = structs.iter().map(|s| s.ident.clone()).collect::<Vec<_>>();

    output.extend(quote! {
        pub trait Visitor {
            type Res;
            #(
                fn #visit_method_names(&self, expr: &#visit_idents) -> Result<Self::Res, LoxError>;
            )*
        }
    });

    // define `Expr` trait
    output.extend(quote! {
        pub trait #base_ident {
            fn accept(&self, visitor: Box<&dyn Visitor>) -> Result<Visitor::Res, LoxError>;
        }
    });

    // define structs.
    structs.iter().for_each(|s| {
        let ident = &s.ident;
        let visit_method_name = format_ident!("visit_{}", s.get_ident_name_lowercase());
        let trait_field_names = s
            .fields
            .iter()
            .filter(|f| f.is_trait)
            .map(|f| f.name.clone())
            .collect::<Vec<_>>();
        let trait_field_tys = s
            .fields
            .iter()
            .filter(|f| f.is_trait)
            .map(|f| f.ty.clone())
            .collect::<Vec<_>>();

        let field_names = s
            .fields
            .iter()
            .filter(|f| !f.is_trait)
            .map(|f| f.name.clone())
            .collect::<Vec<_>>();
        let field_tys = s
            .fields
            .iter()
            .filter(|f| !f.is_trait)
            .map(|f| f.ty.clone())
            .collect::<Vec<_>>();

        output.extend(quote! {
                pub struct #ident {
                    #(
                        pub #trait_field_names: Box<dyn #trait_field_tys>,
                    )*
                    #(
                        pub #field_names: #field_tys,
                    )*
                }

                impl #base_ident for #ident {
                    fn accept(&self, visitor: Box<&dyn Visitor>) -> Result<Visitor::Res, LoxError> {
                        visitor.#visit_method_name(self)
                    }
                }
        });
    });
    output.into()
}
