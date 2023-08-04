use proc_macro::TokenStream;
mod gen;

#[proc_macro]
pub fn define_ast(input: TokenStream) -> TokenStream {
    gen::gen_ast(input)
}
