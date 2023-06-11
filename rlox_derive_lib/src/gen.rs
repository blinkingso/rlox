use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{
    parse::Parse, parse_macro_input, spanned::Spanned, token::Comma, Expr, ExprArray, Lit, LitStr,
    Type,
};

pub(crate) struct ManagerExprItem {
    output_dir: Lit,
    base_name: Ident,
    base_name_ty: Type,
    types: Vec<ClassItem>,
}

pub(crate) struct ClassItem {
    class_name: Ident,
    class_name_ty: Type,
    fields: Vec<FieldItem>,
}

pub(crate) struct FieldItem {
    field_name: Ident,
    field_ty_name: Ident,
    field_ty_ty: Type,
}

impl Parse for ManagerExprItem {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // parse output dir literal
        let output_dir: Lit = input.parse()?;
        input.parse::<Comma>()?;
        // parse base_name ident
        let base_name: Lit = input.parse()?;
        let base_name_str = if let Lit::Str(lit) = base_name {
            lit.value()
        } else {
            panic!("base name should be literal!");
        };
        let base_name = Ident::new(base_name_str.as_str(), Span::call_site());
        let base_name_ty = syn::parse_str(base_name_str.as_str())?;
        input.parse::<Comma>()?;
        // parse types
        let classes: ExprArray = input.parse()?;
        let mut class_items = vec![];
        for ele in classes.elems {
            if let Expr::Lit(class) = ele {
                if let Lit::Str(ref lit) = class.lit {
                    let classes = lit.value();
                    let (class_name, fields) = classes.split_once(":").ok_or(syn::Error::new(
                        classes.span(),
                        "expeceted {Class}: {Fields}",
                    ))?;
                    let class_name = class_name.trim();
                    let fields = fields.trim();
                    let fields = fields.split(",");
                    let mut field_items: Vec<FieldItem> = vec![];
                    for field in fields.into_iter() {
                        let field = field.trim();
                        let (field_ty, field_name) =
                            field.split_once(" ").ok_or(syn::Error::new(
                                class.span(),
                                "Expected fields {FieldType}:{FieldName} eg: Expr left",
                            ))?;
                        let field_ty = field_ty.trim();
                        let field_name = field_name.trim();
                        let field_item = FieldItem {
                            field_name: Ident::new(field_name, Span::call_site()),
                            field_ty_name: Ident::new(field_ty, Span::call_site()),
                            field_ty_ty: syn::parse_str(field_ty)?,
                        };
                        field_items.push(field_item);
                    }
                    class_items.push(ClassItem {
                        class_name: Ident::new(class_name, Span::call_site()),
                        class_name_ty: syn::parse_str(class_name)?,
                        fields: field_items,
                    });
                } else {
                    panic!("classes definition must be string literal!");
                }
            } else {
                panic!("classes definition must be string literal!");
            }
        }
        Ok(ManagerExprItem {
            output_dir,
            base_name,
            base_name_ty,
            types: class_items,
        })
    }
}

pub(crate) fn gen_ast(input: TokenStream) -> TokenStream {
    let ManagerExprItem {
        base_name, types, ..
    } = parse_macro_input!(input as ManagerExprItem);
    let enum_types = types
        .iter()
        .map(|ClassItem { class_name, .. }| quote!(quote!(#class_name)(quote!(#class_name)quote!(#base_name))))
        .collect::<Vec<_>>();
    println!("{}", base_name.to_string());
    let output = quote! {
        // use rlox::literal::*;
        // use rlox::token::*;

        pub enum #base_name {
            // #(#enum_types),*
            Str
        }
    };
    output.into()
}
