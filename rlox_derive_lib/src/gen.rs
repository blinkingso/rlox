use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{format_ident, quote};
use syn::{
    parse::Parse, parse_macro_input, spanned::Spanned, token::Comma, Expr, ExprArray, Lit, Type,
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
    struct_name: Ident,
    struct_name_ty: Type,
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
                    let struct_name = format!("{}{}", class_name, base_name);
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
                        struct_name: Ident::new(struct_name.as_str(), Span::call_site()),
                        struct_name_ty: syn::parse_str(struct_name.as_str())?,
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
        .map(
            |ClassItem {
                 class_name,
                 struct_name,
                 ..
             }| quote!(#class_name(Box<#struct_name>)),
        )
        .collect::<Vec<_>>();

    // gen enum Expr
    let mut output = quote! {
        use crate::literal::*;
        use crate::token::*;

        pub enum #base_name {
            #(#enum_types),*
        }
    };

    // gen struct
    types.iter().for_each(
        |ClassItem {
             struct_name,
             fields,
             ..
         }| {
            let fields = fields
                .iter()
                .map(
                    |FieldItem {
                         field_name,
                         field_ty_name,
                         ..
                     }| quote!(pub #field_name: Box<#field_ty_name>),
                )
                .collect::<Vec<_>>();
            output.extend(quote! {
                pub struct #struct_name {
                    #(#fields),*
                }
            });
        },
    );

    // Visitor trait...
    define_visitor(&mut output, &base_name, &types);
    output.into()
}

fn define_visitor(
    output: &mut proc_macro2::TokenStream,
    base_name: &Ident,
    types: &Vec<ClassItem>,
) {
    let base_name_str = base_name.to_string();
    let base_name_ident = format_ident!("{}", base_name_str.to_lowercase());
    let methods = types
        .iter()
        .map(
            |ClassItem {
                 class_name,
                 struct_name_ty,
                 ..
             }| {
                let method_name = format_ident!(
                    "visit_{}_{}",
                    class_name.to_string().to_lowercase(),
                    base_name_str.to_lowercase()
                );
                quote! {fn #method_name(&self, #base_name_ident: Box<#struct_name_ty>) -> std::io::Result<R>;}
            },
        )
        .collect::<Vec<_>>();
    let visitor = quote! {
        pub trait Visitor<R> {
            #(#methods)*
        }
    };
    output.extend(visitor);

    let impls = types
        .iter()
        .map(
            |ClassItem {
                 class_name,
                 struct_name,
                 ..
             }| {
                let call_visitor = format_ident!(
                    "visit_{}_{}",
                    class_name.to_string().to_lowercase(),
                    base_name_str.to_lowercase(),
                );
                quote! {

                    impl #struct_name {
                        pub fn accept<R>(self:Box<Self>, visitor: Box<&dyn Visitor<R>>) -> std::io::Result<R> {
                            visitor.#call_visitor(self)
                        }
                    }

                }
            },
        )
        .collect::<Vec<_>>();

    output.extend(quote! {

        #(#impls)*
    });
}
