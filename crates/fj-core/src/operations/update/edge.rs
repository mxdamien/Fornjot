use fj_math::Point;

use crate::{
    geometry::{CurveBoundary, SurfacePath},
    objects::{Curve, HalfEdge, Vertex},
    storage::Handle,
};

/// Update a [`HalfEdge`]
pub trait UpdateHalfEdge {
    /// Replace the path of the half-edge
    #[must_use]
    fn replace_path(&self, path: SurfacePath) -> Self;

    /// Replace the boundary of the half-edge
    #[must_use]
    fn replace_boundary(&self, boundary: CurveBoundary<Point<1>>) -> Self;

    /// Replace the curve of the half-edge
    #[must_use]
    fn replace_curve(&self, curve: Handle<Curve>) -> Self;

    /// Replace the start vertex of the half-edge
    #[must_use]
    fn replace_start_vertex(&self, start_vertex: Handle<Vertex>) -> Self;
}

impl UpdateHalfEdge for HalfEdge {
    fn replace_path(&self, path: SurfacePath) -> Self {
        HalfEdge::new(
            path,
            self.boundary(),
            self.curve().clone(),
            self.start_vertex().clone(),
            self.global_form().clone(),
        )
    }

    fn replace_boundary(&self, boundary: CurveBoundary<Point<1>>) -> Self {
        HalfEdge::new(
            self.path(),
            boundary,
            self.curve().clone(),
            self.start_vertex().clone(),
            self.global_form().clone(),
        )
    }

    fn replace_curve(&self, curve: Handle<Curve>) -> Self {
        HalfEdge::new(
            self.path(),
            self.boundary(),
            curve,
            self.start_vertex().clone(),
            self.global_form().clone(),
        )
    }

    fn replace_start_vertex(&self, start_vertex: Handle<Vertex>) -> Self {
        HalfEdge::new(
            self.path(),
            self.boundary(),
            self.curve().clone(),
            start_vertex,
            self.global_form().clone(),
        )
    }
}
