pub mod swept;

pub use self::swept::SweptCurve;

use fj_math::{Point, Transform, Vector};

use crate::geometry;

use super::Curve;

/// A two-dimensional shape
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Surface {
    /// A swept curve
    SweptCurve(SweptCurve),
}

impl Surface {
    /// Construct a `Surface` that represents the xy-plane
    pub fn xy_plane() -> Self {
        Self::SweptCurve(SweptCurve {
            curve: Curve::x_axis(),
            path: Vector::unit_y(),
        })
    }

    /// Construct a `Surface` that represents the xz-plane
    pub fn xz_plane() -> Self {
        Self::SweptCurve(SweptCurve {
            curve: Curve::x_axis(),
            path: Vector::unit_z(),
        })
    }

    /// Construct a `Surface` that represents the yz-plane
    pub fn yz_plane() -> Self {
        Self::SweptCurve(SweptCurve {
            curve: Curve::y_axis(),
            path: Vector::unit_z(),
        })
    }

    /// Create a new instance that is reversed
    #[must_use]
    pub fn reverse(self) -> Self {
        match self {
            Self::SweptCurve(surface) => Self::SweptCurve(surface.reverse()),
        }
    }

    /// Transform the surface
    #[must_use]
    pub fn transform(self, transform: &Transform) -> Self {
        match self {
            Self::SweptCurve(surface) => {
                Self::SweptCurve(surface.transform(transform))
            }
        }
    }

    /// Convert a point in model coordinates to surface coordinates
    pub fn convert_point_to_surface_coords(
        &self,
        point_3d: impl Into<Point<3>>,
    ) -> geometry::Point<2> {
        let point_3d = point_3d.into();

        let point_2d = match self {
            Self::SweptCurve(surface) => {
                surface.point_to_surface_coords(point_3d)
            }
        };

        geometry::Point::new(point_2d, point_3d)
    }

    /// Convert a point in surface coordinates to model coordinates
    pub fn convert_point_from_surface_coords(
        &self,
        point: impl Into<Point<2>>,
    ) -> Point<3> {
        match self {
            Self::SweptCurve(surface) => {
                surface.point_from_surface_coords(point)
            }
        }
    }

    /// Convert a vector in surface coordinates to model coordinates
    pub fn convert_vector_from_surface_coords(
        &self,
        vector: impl Into<Vector<2>>,
    ) -> Vector<3> {
        match self {
            Self::SweptCurve(surface) => {
                surface.vector_from_surface_coords(vector)
            }
        }
    }
}
