use anyhow::Result;

use opencv::core::*;
use opencv::prelude::*;

// use opencv_ext::MyIntoResult;

#[test]
fn test_expr_macro_opencv() -> Result<()> {
    let a = Mat::new_rows_cols_with_default(2, 3, CV_8UC1, 100.0.into())?;
    let b = Mat::new_rows_cols_with_default(2, 3, CV_8UC1, 10.0.into())?;
    let c = Mat::new_rows_cols_with_default(2, 3, CV_8UC1, 50.0.into())?;
    let d = Mat::new_rows_cols_with_default(2, 3, CV_8UC1, 2.0.into())?;
    // let c: Mat = (failable_expr! { a - b }).to_mat()?;
    // let c: Mat = ((((a).into_result()?) - ((b).into_result()?)).into_result()?).to_mat()?;
    // let c: Mat = (a.into_result()? - b.into_result()?).into_result()?.to_mat()?;

    // ok
    // let a_sub_b: MatExpr = (a - b).my_into_result()?;
    // let a_sub_b_sub_c: MatExpr = (a_sub_b - c).my_into_result()?;
    // let a_sub_b_sub_c_mat: Mat = a_sub_b_sub_c.to_mat()?;

    // not ok
    // let a_sub_b = (a - b).my_into_result()?;
    // let a_sub_b_sub_c: MatExpr = (a_sub_b - c).my_into_result()?;

    // ok
    // let a_sub_b = (a - b)?;
    // let a_sub_b_sub_c: MatExpr = (a_sub_b - c)?;

    // let a_sub_b = a - b;
    // let a_sub_b_sub_c = a_sub_b - c;
    // let a_sub_b_sub_c_ans = a_sub_b_sub_c.into_result()?;

    // let u = &a - &b;
    // let x = ((&a - &b) - (&c - &d)).into_result()?.to_mat()?;
    let x = (&a / d - &b + &c - 10f64).into_result()?.to_mat()?;
    println!("x={:?}", x.data_bytes());

    // assert_eq!(
    //     c.data_bytes()?,
    //     Mat::new_rows_cols_with_default(3, 5, CV_8UC1, 80.0.into())?.data_bytes()?,
    // );
    Ok(())
}

// #[test]
// fn test_expr_macro_simple() -> Result<()> {
//     let a = 42;
//     let b = 100;
//     let c = 10u8;
//     assert_eq!(failable_expr! { a + b - c }, 132,);
//     Ok(())
// }
