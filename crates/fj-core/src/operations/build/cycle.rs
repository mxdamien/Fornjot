use fj_math::{Point, Scalar};
use itertools::Itertools;

use crate::{
    objects::{Cycle, HalfEdge},
    operations::{build::BuildHalfEdge, update::UpdateCycle},
    Core,
};

/// Build a [`Cycle`]
///
/// See [module-level documentation] for context.
///
/// [module-level documentation]: super
pub trait BuildCycle {
    /// Build an empty cycle
    fn empty() -> Cycle {
        Cycle::new([])
    }

    /// Build a circle
    fn circle(
        center: impl Into<Point<2>>,
        radius: impl Into<Scalar>,
        core: &mut Core,
    ) -> Cycle {
        let circle = HalfEdge::circle(center, radius, core);
        Cycle::empty().add_half_edges([circle], core)
    }

    /// Build a polygon
    fn polygon<P, Ps>(points: Ps, core: &mut Core) -> Cycle
    where
        P: Into<Point<2>>,
        Ps: IntoIterator<Item = P>,
        Ps::IntoIter: Clone + ExactSizeIterator,
    {
        let edges = points
            .into_iter()
            .map(Into::into)
            .circular_tuple_windows()
            .map(|(start, end)| {
                HalfEdge::line_segment([start, end], None, core)
            });

        Cycle::new(edges)
    }
}

impl BuildCycle for Cycle {}
