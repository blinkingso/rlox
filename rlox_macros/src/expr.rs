use proc_macro2::Span;
use syn::{parse::Parse, token::Comma, Expr, ExprArray, Ident, Lit, LitStr};

#[derive(Debug)]
pub struct ExprAst {
    pub ident: Ident,
    pub structs: Vec<StructItem>,
}

#[derive(Debug)]
pub struct StructItem {
    pub ident: Ident,
    pub fields: Vec<FieldItem>,
}

impl StructItem {
    pub fn get_ident_name_lowercase(&self) -> String {
        self.ident.to_string().to_lowercase()
    }
}

#[derive(Debug)]
pub struct FieldItem {
    pub is_trait: bool,
    pub name: Ident,
    pub ty: Ident,
}

impl Parse for ExprAst {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let base_ident: LitStr = input.parse()?;
        let base_ident = Ident::new(base_ident.value().to_string().as_str(), Span::call_site());
        let _ = input.parse::<Comma>()?;
        let structs: ExprArray = input.parse()?;

        let structs = structs
            .elems
            .iter()
            .map(|ele| {
                if let Expr::Lit(lit) = ele {
                    if let Lit::Str(lit) = &lit.lit {
                        let value = lit.value();
                        let (struct_name, fields) = value.trim().split_once(":").unwrap();
                        let ident = Ident::new(struct_name.trim(), Span::call_site());

                        let fields = fields
                            .split(",")
                            .into_iter()
                            .map(|field| {
                                let (field_ty, field_name) = field.trim().split_once(" ").unwrap();
                                let is_trait = field_ty.eq(base_ident.to_string().as_str());
                                FieldItem {
                                    is_trait,
                                    name: Ident::new(field_name.trim(), Span::call_site()),
                                    ty: Ident::new(field_ty.trim(), Span::call_site()),
                                }
                            })
                            .collect();

                        return StructItem { ident, fields };
                    }
                }
                panic!("Illegal expr format");
            })
            .collect();
        Ok(ExprAst {
            ident: base_ident,
            structs,
        })
    }
}
