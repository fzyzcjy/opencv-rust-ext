use anyhow::Result;
use opencv::core::CV_8UC1;
use opencv::prelude::*;

use opencv_ext::failable_expr;
use opencv_ext::IntoResult;

// does not compile yet
// #[test]
// fn test_expr_macro_opencv() -> Result<()> {
//     let a = Mat::new_rows_cols_with_default(3, 5, CV_8UC1, 42.0.into())?;
//     let b = Mat::new_rows_cols_with_default(3, 5, CV_8UC1, 100.0.into())?;
//     let c = 10u8;
//     assert_eq!(
//         failable_expr! { a + b - c },
//         Mat::new_rows_cols_with_default(3, 5, CV_8UC1, 132.0.into())
//     );
//     Ok(())
// }

#[test]
fn test_expr_macro_simple() -> Result<()> {
    let a = 42;
    let b = 100;
    let c = 10u8;
    assert_eq!(failable_expr! { a + b - c }, 132,);
    Ok(())
}
