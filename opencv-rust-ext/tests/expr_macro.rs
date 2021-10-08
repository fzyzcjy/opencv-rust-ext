use anyhow::Result;

use opencv_ext::failable_expr;
use opencv_ext::IntoResult;

#[test]
fn test_expr_macro() -> Result<()> {
    let a = 42;
    let b = 100;
    let c = 50;

    let result = failable_expr! { a - b / c };

    // let result = failable_expr! { a - b };
    // let result = ((((a.into_result())?) - ((b.into_result())?).into_result())?);

    // let result = failable_expr! { a };
    // let result = ((a.into_result())?);

    println!("result={}", result);
    Ok(())
}
