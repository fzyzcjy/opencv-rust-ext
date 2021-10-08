use proc_macro::*;

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
    // ref: https://github.com/fzyzcjy/yplusplus/issues/1073#issuecomment-938616160
    fn fold_expr(&mut self, expr: Expr) -> Expr {
        let child = fold_expr(self, expr);
        Expr::Try(ExprTry {
            attrs: Vec::new(),
            expr: Box::new(Expr::MethodCall(ExprMethodCall {
                attrs: vec![],
                receiver: Box::new(child),
                dot_token: Default::default(),
                method: Ident::new("into_result", Span::call_site()),
                turbofish: None,
                paren_token: Default::default(),
                args: vec![],
            })),
            question_token: Default::default(),
        })
    }
}
