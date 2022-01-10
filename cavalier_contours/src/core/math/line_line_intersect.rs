use super::{base_math::parametric_from_point, Vector2};
use crate::core::traits::Real;

/// Holds the result of finding the intersect between two line segments.
#[derive(Debug, Copy, Clone)]
pub enum LineLineIntr<T>
where
    T: Real,
{
    /// No intersect, segments are parallel and not collinear.
    NoIntersect,
    /// There is a true intersect between the line segments.
    TrueIntersect {
        /// Parametric value for intersect on first segment.
        seg1_t: T,
        /// Parametric value for intersect on second segment.
        seg2_t: T,
    },
    /// Segments overlap each other (are collinear) by some amount.
    Overlapping {
        /// Parametric value for start of coincidence along second segment.
        seg2_t0: T,
        /// Parametric value for end of coincidence along second segment.
        seg2_t1: T,
    },
    /// There is an intersect between the lines but one or both of the segments must be extended.
    FalseIntersect {
        /// Parametric value for intersect on first segment.
        seg1_t: T,
        /// Parametric value for intersect on second segment.
        seg2_t: T,
    },
}

/// Finds the intersects between two lines segments.
///
/// Line segments are defined by `v1->v2` and `u1->u2`.
/// Handles the cases where the lines may be parallel, collinear, or single points.
pub fn line_line_intr<T>(
    v1: Vector2<T>,
    v2: Vector2<T>,
    u1: Vector2<T>,
    u2: Vector2<T>,
    epsilon: T,
) -> LineLineIntr<T>
where
    T: Real,
{
    // This implementation works by processing the segments in parametric equation form and using
    // perpendicular products
    // http://geomalgorithms.com/a05-_intersect-1.html
    // http://mathworld.wolfram.com/PerpDotProduct.html

    use LineLineIntr::*;

    let v = v2 - v1;
    let u = u2 - u1;
    let v_pdot_u = v.perp_dot(u);
    let w = v1 - u1;

    let eps = epsilon;

    // threshold check here to avoid almost parallel lines resulting in very distant intersection
    if !v_pdot_u.fuzzy_eq_zero_eps(eps) {
        // segments not parallel or collinear
        let seg1_t = u.perp_dot(w) / v_pdot_u;
        let seg2_t = v.perp_dot(w) / v_pdot_u;
        if !seg1_t.fuzzy_in_range_eps(T::zero(), T::one(), eps)
            || !seg2_t.fuzzy_in_range_eps(T::zero(), T::one(), eps)
        {
            return FalseIntersect { seg1_t, seg2_t };
        }
        return TrueIntersect { seg1_t, seg2_t };
    }

    // segments are parallel and possibly collinear
    let v_pdot_w = v.perp_dot(w);
    let u_pdot_w = u.perp_dot(w);

    // threshold check here, we consider almost parallel lines to be parallel
    if !v_pdot_w.fuzzy_eq_zero_eps(eps) || !u_pdot_w.fuzzy_eq_zero_eps(eps) {
        // parallel and not collinear so no intersect
        return NoIntersect;
    }

    // either collinear or degenerate (segments are single points)
    let v_is_point = v1.fuzzy_eq_eps(v2, eps);
    let u_is_point = u1.fuzzy_eq_eps(u2, eps);

    if v_is_point && u_is_point {
        // both segments are points
        if v1.fuzzy_eq_eps(u1, eps) {
            // same point
            return TrueIntersect {
                seg1_t: T::zero(),
                seg2_t: T::zero(),
            };
        }
        // distinct points
        return NoIntersect;
    }

    if v_is_point {
        // v is point and u is not a point
        let seg2_t = parametric_from_point(u1, u2, v1);
        if seg2_t.fuzzy_in_range_eps(T::zero(), T::one(), eps) {
            return TrueIntersect {
                seg1_t: T::zero(),
                seg2_t,
            };
        }

        return NoIntersect;
    }

    if u_is_point {
        // u is point and v is not a point
        let seg1_t = parametric_from_point(v1, v2, u1);
        if seg1_t.fuzzy_in_range_eps(T::zero(), T::one(), eps) {
            return TrueIntersect {
                seg1_t,
                seg2_t: T::zero(),
            };
        }

        return NoIntersect;
    }

    // neither segment is a point, check if they overlap
    let w2 = v2 - u1;
    let (mut seg2_t0, mut seg2_t1) = if u.x.fuzzy_eq_zero_eps(eps) {
        (w.y / u.y, w2.y / u.y)
    } else {
        (w.x / u.x, w2.x / u.x)
    };

    if seg2_t0 > seg2_t1 {
        std::mem::swap(&mut seg2_t0, &mut seg2_t1);
    }

    // using threshold check here to make intersect "sticky" to prefer considering it an intersect
    if !seg2_t0.fuzzy_lt_eps(T::one(), eps) || !seg2_t1.fuzzy_gt_eps(T::zero(), eps) {
        return NoIntersect;
    }

    seg2_t0 = num_traits::real::Real::max(seg2_t0, T::zero());
    seg2_t1 = num_traits::real::Real::min(seg2_t1, T::one());

    if (seg2_t1 - seg2_t0).fuzzy_eq_zero_eps(eps) {
        // intersect is a single point (segments line up end to end)
        // determine if seg1_t is 0.0 or 1.0
        let seg1_t = if v1.fuzzy_eq_eps(u1, eps) || v1.fuzzy_eq_eps(u2, eps) {
            // v1 touches which is start of seg1
            T::zero()
        } else {
            T::one()
        };

        return TrueIntersect {
            seg1_t,
            seg2_t: seg2_t0,
        };
    }

    Overlapping { seg2_t0, seg2_t1 }
}
