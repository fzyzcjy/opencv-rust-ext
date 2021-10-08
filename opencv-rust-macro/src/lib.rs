use proc_macro::TokenStream;

use quote::quote;
use syn::fold::{fold_expr, Fold};
use syn::*;

#[proc_macro]
pub fn failable_expr(input: TokenStream) -> TokenStream {
    let src_ast: Expr = parse_macro_input!(input);
    println!("opencv_expr src_ast={:#?}", src_ast);

    let ans_ast = OpencvExpr.fold_expr(src_ast);
    println!("opencv_expr ans_ast={:#?}", ans_ast);

    quote!(#ans_ast).into()
}

struct OpencvExpr;

impl Fold for OpencvExpr {
    // ref: https://docs.rs/syn/1.0.80/syn/fold/index.html#example
    fn fold_expr(&mut self, expr: Expr) -> Expr {
        Expr::Paren(ExprParen {
            attrs: Vec::new(),
            expr: Box::new(fold_expr(self, expr)),
            paren_token: token::Paren::default(),
        })
    }
}
