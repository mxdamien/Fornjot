use fj_math::Point;
use itertools::Itertools;

use crate::{
    geometry::curve::Curve,
    insert::Insert,
    objects::{Cycle, HalfEdge, Objects},
    services::Service,
    storage::Handle,
};

use super::{HalfEdgeBuilder, ObjectArgument};

/// Builder API for [`Cycle`]
pub trait CycleBuilder: Sized {
    /// Add a new half-edge to the cycle
    ///
    /// Creates a half-edge and adds it to the cycle. The new half-edge is
    /// connected to the front vertex of the last half-edge , and the back
    /// vertex of the first edge, making sure the half-edges actually form a
    /// cycle.
    ///
    /// If this is the first half-edge being added, it is connected to itself,
    /// meaning its front and back vertices are the same.
    fn add_half_edge(
        self,
        half_edge: HalfEdgeBuilder,
        objects: &mut Service<Objects>,
    ) -> (Self, Handle<HalfEdge>);

    /// Update cycle as a polygon from the provided points
    fn update_as_polygon_from_points<P, Ps>(
        self,
        points: Ps,
        objects: &mut Service<Objects>,
    ) -> Self
    where
        P: Into<Point<2>>,
        Ps: IntoIterator<Item = P>,
        Ps::IntoIter: Clone + ExactSizeIterator;

    /// Connect the cycles to the provided half-edges
    ///
    /// Assumes that the provided half-edges, once translated into local
    /// equivalents of this cycle, form a cycle themselves.
    ///
    /// Returns the local equivalents of the provided half-edges.
    fn connect_to_edges<O>(
        self,
        edges: O,
        objects: &mut Service<Objects>,
    ) -> (Self, O::SameSize<Handle<HalfEdge>>)
    where
        O: ObjectArgument<(Handle<HalfEdge>, Curve, [Point<1>; 2])>;
}

impl CycleBuilder for Cycle {
    fn add_half_edge(
        self,
        half_edge: HalfEdgeBuilder,
        objects: &mut Service<Objects>,
    ) -> (Self, Handle<HalfEdge>) {
        let half_edge = half_edge.build(objects).insert(objects);
        let cycle =
            Cycle::new(self.half_edges().cloned().chain([half_edge.clone()]));
        (cycle, half_edge)
    }

    fn update_as_polygon_from_points<P, Ps>(
        mut self,
        points: Ps,
        objects: &mut Service<Objects>,
    ) -> Self
    where
        P: Into<Point<2>>,
        Ps: IntoIterator<Item = P>,
        Ps::IntoIter: Clone + ExactSizeIterator,
    {
        points
            .into_iter()
            .map(Into::into)
            .circular_tuple_windows()
            .for_each(|(start, end)| {
                let half_edge =
                    HalfEdgeBuilder::line_segment([start, end], None);

                let (cycle, _) = self.clone().add_half_edge(half_edge, objects);
                self = cycle;
            });

        self
    }

    fn connect_to_edges<O>(
        mut self,
        edges: O,
        objects: &mut Service<Objects>,
    ) -> (Self, O::SameSize<Handle<HalfEdge>>)
    where
        O: ObjectArgument<(Handle<HalfEdge>, Curve, [Point<1>; 2])>,
    {
        let edges =
            edges.map_with_prev(|(_, curve, boundary), (prev, _, _)| {
                let half_edge = HalfEdgeBuilder::new(curve, boundary)
                    .with_start_vertex(prev.start_vertex().clone());

                let (cycle, half_edge) =
                    self.clone().add_half_edge(half_edge, objects);
                self = cycle;

                half_edge
            });

        (self, edges)
    }
}
