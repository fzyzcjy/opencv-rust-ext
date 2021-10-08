use proc_macro::TokenStream;

use quote::quote;
use syn::fold::{fold_expr, Fold};
use syn::*;

#[proc_macro]
pub fn failable_expr(input: TokenStream) -> TokenStream {
    let src_ast: Expr = parse_macro_input!(input);
    println!("opencv_expr src_ast={:#?}", src_ast);
    println!("opencv_expr src_ast.quote={}", quote! {#src_ast});

    let ans_ast = OpencvExpr.fold_expr(src_ast);
    println!("opencv_expr ans_ast={:#?}", ans_ast);
    println!("opencv_expr ans_ast.quote={}", quote! {#ans_ast});

    quote!(#ans_ast).into()
}

struct OpencvExpr;

impl Fold for OpencvExpr {
    // ref: https://docs.rs/syn/1.0.80/syn/fold/index.html#example
    // ref: https://github.com/fzyzcjy/yplusplus/issues/1073#issuecomment-938616160
    fn fold_expr(&mut self, expr: Expr) -> Expr {
        let child = fold_expr(self, expr);
        Expr::Paren(ExprParen {
            attrs: vec![],
            paren_token: Default::default(),
            expr: Box::new(Expr::Try(ExprTry {
                attrs: Vec::new(),
                expr: Box::new(Expr::MethodCall(ExprMethodCall {
                    attrs: vec![],
                    receiver: Box::new(Expr::Paren(ExprParen {
                        attrs: vec![],
                        paren_token: Default::default(),
                        expr: Box::new(child),
                    })),
                    dot_token: Default::default(),
                    method: proc_macro2::Ident::new("into_result", proc_macro2::Span::call_site()),
                    turbofish: None,
                    paren_token: Default::default(),
                    args: Default::default(),
                })),
                question_token: Default::default(),
            })),
        })
    }
}
