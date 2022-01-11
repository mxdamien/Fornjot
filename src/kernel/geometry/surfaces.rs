use crate::math::Point;

/// A two-dimensional shape
pub enum Surface {
    /// The X-Y plane
    ///
    /// This will be replaced with a more general plane representation in due
    /// time.
    XYPlane,
}

impl Surface {
    /// Convert a point in model coordinates to surface coordinates
    pub fn model_to_surface(&self, mut point: Point) -> Point {
        point.z = 0.;
        point
    }

    /// Convert a point in surface coordinates to model coordinates
    pub fn surface_to_model(&self, point: Point) -> Point {
        // We're temporarily using the same point type for model and surface
        // coordinates, so there's nothing to do here.
        point
    }
}
