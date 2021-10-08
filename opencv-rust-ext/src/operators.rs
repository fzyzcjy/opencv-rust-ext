pub trait IntoResult<T, E> {
    fn into_result(self) -> Result<T, E>;
}

impl<T, E> IntoResult<T, E> for Result<T, E> {
    fn into_result(self) -> Result<T, E> {
        self
    }
}

impl<T> IntoResult<T, ()> for T {
    fn into_result(self) -> Result<T, ()> {
        Ok(self)
    }
}

#[cfg(test)]
mod test {
    use crate::IntoResult;

    #[test]
    fn test_into_result() {
        assert_eq!(Ok(42), 42.into_result());
        assert_eq!(Result::<i32, _>::Err("oops"), Err("oops").into_result());
        assert_eq!(Result::<_, &str>::Ok(100), Ok(100).into_result());
    }
}
