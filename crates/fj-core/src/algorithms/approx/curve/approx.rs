use std::collections::VecDeque;

use fj_math::Point;

use crate::geometry::CurveBoundary;

use super::{CurveApproxPoints, CurveApproxSegment};

/// Partial approximation of a curve
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct CurveApprox {
    segments: Vec<(CurveBoundary<Point<1>>, CurveApproxPoints)>,
}

impl CurveApprox {
    /// Get the single segment that covers the provided boundary, if available
    pub fn into_single_segment(
        mut self,
        boundary: CurveBoundary<Point<1>>,
    ) -> Option<CurveApproxSegment> {
        match self.segments.pop() {
            Some((b, points)) if self.segments.is_empty() && b == boundary => {
                // We just removed a single segment, there are no others, and
                // the removed segment's boundary matches the boundary provided
                // to us.
                //
                // This is what the caller was asking for. Return it!
                Some(CurveApproxSegment {
                    boundary: b,
                    points,
                })
            }
            _ => {
                // Either we don't have any segments in here, or we have more
                // than one (which implies there are gaps between them), or we
                // have a single one that doesn't cover the full boundary we
                // were asked for.
                //
                // Either way, we don't have what the caller wants.
                None
            }
        }
    }

    /// Reverse the approximation
    pub fn reverse(&mut self) {
        self.segments.reverse();

        for (boundary, segment) in &mut self.segments {
            *boundary = boundary.reverse();
            segment.reverse();
        }
    }

    /// Reduce the approximation to the subset defined by the provided boundary
    pub fn make_subset(&mut self, boundary: CurveBoundary<Point<1>>) {
        for (b, segment) in &mut self.segments {
            *b = b.subset(boundary);
            segment.make_subset(boundary.normalize());
        }

        self.segments.retain(|(boundary, _)| !boundary.is_empty());
    }

    /// Merge the provided segment into the approximation
    pub fn merge(
        &mut self,
        new_segment: CurveApproxSegment,
    ) -> CurveApproxSegment {
        let mut overlapping_segments = VecDeque::new();

        let mut i = 0;
        loop {
            let Some((boundary, _)) = self.segments.get(i) else {
                break;
            };

            if boundary.overlaps(&new_segment.boundary) {
                let segment = self.segments.swap_remove(i);
                overlapping_segments.push_back(segment);
                continue;
            }

            i += 1;
        }

        let mut merged_boundary = new_segment.boundary;
        let mut merged_segment = new_segment.points;

        for (boundary, segment) in overlapping_segments {
            assert!(
                merged_boundary.overlaps(&boundary),
                "Shouldn't merge segments that don't overlap."
            );

            merged_boundary = merged_boundary.union(boundary);
            merged_segment.merge(&segment, boundary);
        }

        self.segments
            .push((merged_boundary, merged_segment.clone()));
        self.segments.sort();

        CurveApproxSegment {
            boundary: merged_boundary,
            points: merged_segment,
        }
    }
}

impl<const N: usize> From<[CurveApproxSegment; N]> for CurveApprox {
    fn from(segments: [CurveApproxSegment; N]) -> Self {
        Self {
            segments: segments
                .into_iter()
                .map(|segment| (segment.boundary, segment.points))
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::algorithms::approx::{curve::CurveApproxSegment, ApproxPoint};

    use super::CurveApprox;

    #[test]
    fn reverse() {
        let mut approx = CurveApprox::from([
            CurveApproxSegment::from((
                [[0.1], [0.4]].into(),
                [
                    ApproxPoint::new([0.1], [0.1, 0.1, 0.1]),
                    ApproxPoint::new([0.4], [0.4, 0.4, 0.4]),
                ],
            )),
            CurveApproxSegment::from((
                [[0.6], [0.9]].into(),
                [
                    ApproxPoint::new([0.6], [0.6, 0.6, 0.6]),
                    ApproxPoint::new([0.9], [0.9, 0.9, 0.9]),
                ],
            )),
        ]);

        approx.reverse();

        assert_eq!(
            approx,
            CurveApprox::from([
                CurveApproxSegment::from((
                    [[0.9], [0.6]].into(),
                    [
                        ApproxPoint::new([0.9], [0.9, 0.9, 0.9]),
                        ApproxPoint::new([0.6], [0.6, 0.6, 0.6]),
                    ],
                )),
                CurveApproxSegment::from((
                    [[0.4], [0.1]].into(),
                    [
                        ApproxPoint::new([0.4], [0.4, 0.4, 0.4]),
                        ApproxPoint::new([0.1], [0.1, 0.1, 0.1]),
                    ],
                )),
            ])
        )
    }
}