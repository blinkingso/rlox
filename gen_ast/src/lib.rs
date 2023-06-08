use proc_macro::TokenStream;
use quote::quote;
use std::collections::HashMap;
use syn::{parse2, parse_macro_input, Data, DeriveInput};

trait Expr {}

const TYPES: [&'static str; 4] = [
    "Binary     : Expr left, Token operator, Expr right",
    "Grouping   : Expr expression",
    "Literal    : Object value",
    "Unary      : Token operator, Expr right",
];

fn parse_types() -> HashMap<String, HashMap<String, String>> {
    let mut map = HashMap::new();
    for ty in TYPES {
        let mut s0 = ty.split(":");
        let ident = s0.next().unwrap().trim();
        let values = s0.next().unwrap().trim();
        let values = values.split(",");
        let mut v_map = HashMap::new();
        for v in values.into_iter() {
            let mut v = v.split_whitespace();
            let field_type = v.next().unwrap();
            let field_name = v.next().unwrap();
            v_map.insert(field_type.to_string(), field_name.to_string());
        }
        map.insert(ident.to_string(), v_map);
    }
    map
}

#[proc_macro]
pub fn define_ast(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let data = &input.data;
    if let Data::Struct(data) = data {
        let fields = &data.fields;
        fields.iter().for_each(|_f| {
            // println!("field: {}->{:?}", f.ident.unwrap(), f.);
        });
    } else {
        panic!("Expr must be derived on `Struct` types.");
    }
    let expanded = quote! {};

    TokenStream::from(expanded)
}
