use crate::math::Point;

/// A shape's vertices
pub trait Vertices {
    /// Return the shape's vertices
    fn vertices(&self) -> Vec<Point>;
}

impl Vertices for fj::Shape {
    fn vertices(&self) -> Vec<Point> {
        match self {
            Self::Shape2d(shape) => shape.vertices(),
            Self::Shape3d(shape) => shape.vertices(),
        }
    }
}

impl Vertices for fj::Shape2d {
    fn vertices(&self) -> Vec<Point> {
        match self {
            Self::Circle(shape) => shape.vertices(),
            Self::Difference(shape) => shape.vertices(),
            Self::Square(shape) => shape.vertices(),
        }
    }
}

impl Vertices for fj::Shape3d {
    fn vertices(&self) -> Vec<Point> {
        match self {
            Self::Sweep(shape) => shape.vertices().into_iter().collect(),
        }
    }
}
