// use std::error::Error;
// use std::fmt::{Debug, Display, Formatter};
//
// pub trait IntoResult<T, E> {
//     fn into_result(self) -> Result<T, E>;
// }
//
// impl<T, E> IntoResult<T, E> for Result<T, E> {
//     fn into_result(self) -> Result<T, E> {
//         self
//     }
// }
//
// impl<T> IntoResult<T, IntoResultDummyError> for T {
//     fn into_result(self) -> Result<T, IntoResultDummyError> {
//         Ok(self)
//     }
// }
//
// #[derive(Debug, Clone, Eq, PartialEq)]
// pub struct IntoResultDummyError;
//
// impl Display for IntoResultDummyError {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(f, "IntoResultDummyError")
//     }
// }
//
// impl Error for IntoResultDummyError {}
//
// #[cfg(test)]
// mod test {
//     use crate::IntoResult;
//
//     #[test]
//     fn test_into_result() {
//         assert_eq!(Ok(42), 42.into_result());
//         assert_eq!(Result::<i32, _>::Err("oops"), Err("oops").into_result());
//         assert_eq!(Result::<_, &str>::Ok(100), Ok(100).into_result());
//     }
// }
