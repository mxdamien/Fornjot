pub type Point = nalgebra::Point<f64, 3>;

/// A point in a surface
///
/// Once default parameters for const generics are stable, it might make more
/// sense to remove this type, and add a `const D: usize = 3` argument to
/// `Point` instead.
pub type Point2 = nalgebra::Point<f64, 2>;

pub type Vector = nalgebra::SVector<f64, 3>;
