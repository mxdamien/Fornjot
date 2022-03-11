pub mod geometry;
pub mod handle;
pub mod topology;
pub mod validate;

pub use self::validate::{ValidationError, ValidationResult};

use crate::math::Scalar;

use super::topology::{
    edges::{Cycle, Edge},
    faces::Face,
    vertices::Vertex,
};

use self::{geometry::Geometry, handle::Storage, topology::Topology};

/// The boundary representation of a shape
#[derive(Clone, Debug)]
pub struct Shape {
    /// The minimum distance between two vertices
    ///
    /// Use for vertex validation, to determine whether vertices are unique.
    min_distance: Scalar,

    vertices: Vertices,
    edges: Edges,
    cycles: Cycles,
    faces: Faces,
}

impl Shape {
    /// Construct a new shape
    pub fn new() -> Self {
        Self {
            // This should really come from `Self::DEFAULT_MIN_DISTANCE`, or a
            // similarly named constant. Unfortunately `Scalar::from_f64` can't
            // be `const` yet.
            min_distance: Scalar::from_f64(5e-7), // 0.5 µm

            vertices: Vertices::new(),
            edges: Edges::new(),
            cycles: Cycles::new(),
            faces: Faces::new(),
        }
    }

    /// Override the minimum distance for this shape
    ///
    /// # Implementation note
    ///
    /// This functionality should be exposed to models, eventually. For now it's
    /// just used in unit tests.
    #[cfg(test)]
    pub fn with_min_distance(
        mut self,
        min_distance: impl Into<Scalar>,
    ) -> Self {
        self.min_distance = min_distance.into();
        self
    }

    /// Access the shape's geometry
    pub fn geometry(&mut self) -> Geometry {
        Geometry
    }

    /// Access the shape's topology
    pub fn topology(&mut self) -> Topology {
        Topology {
            min_distance: self.min_distance,

            geometry: Geometry,

            vertices: &mut self.vertices,
            edges: &mut self.edges,
            cycles: &mut self.cycles,
            faces: &mut self.faces,
        }
    }
}

type Vertices = Vec<Storage<Vertex>>;
type Edges = Vec<Storage<Edge>>;
type Cycles = Vec<Storage<Cycle>>;
type Faces = Vec<Storage<Face>>;