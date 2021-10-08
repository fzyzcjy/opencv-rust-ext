use proc_macro::TokenStream;

use syn::*;

#[proc_macro]
pub fn opencv_expr(input: TokenStream) -> TokenStream {
    let tmp = input.clone();
    let ast: Expr = parse_macro_input!(input);

    println!("ast={:?}", ast);

    tmp
}
