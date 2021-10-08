use opencv::core::*;
use opencv::prelude::*;

pub trait FromPoint<T: ValidPointType + AsPrimitive<S>, S: 'static + ValidPointType> {
    fn from_point(src: &Point_<T>) -> Self;
}

impl<T: ValidPointType + AsPrimitive<S>, S: 'static + ValidPointType> FromPoint<T, S>
for Point_<S>
{
    fn from_point(src: &Point_<T>) -> Self {
        Self {
            x: src.x.as_(),
            y: src.y.as_(),
        }
    }
}

#[cfg(test)]
mod tests {
    use opencv::core::{Point2f, Point2i};

    use crate::FromPoint;

    #[test]
    fn from_point() {
        assert_eq!(Point2f::new(42.0, 100.0), Point2f::from_point(&Point2i::new(42, 100)));
    }
}
