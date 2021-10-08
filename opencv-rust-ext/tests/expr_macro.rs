use anyhow::Result;
use opencv::core::CV_8UC1;
use opencv::prelude::*;

use opencv_ext::failable_expr;
use opencv_ext::IntoResult;

#[test]
fn test_expr_macro() -> Result<()> {
    let a = Mat::new_rows_cols_with_default(3, 5, CV_8UC1, 42.0.into())?;
    let b = Mat::new_rows_cols_with_default(3, 5, CV_8UC1, 100.0.into())?;
    let c = 10u8;

    assert_eq!(
        failable_expr! { a + b - c },
        Mat::new_rows_cols_with_default(3, 5, CV_8UC1, 132.0.into())
    );

    Ok(())
}
