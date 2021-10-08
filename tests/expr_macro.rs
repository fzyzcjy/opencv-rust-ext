use anyhow::Result;

use opencv_ext::opencv_expr;

#[test]
fn test_expr_macro() -> Result<()> {
    let a = 42;
    let b = 100;
    let c = 50;
    let result = opencv_expr! { a - b / c };
    println!("result={}", result);
    Ok(())
}