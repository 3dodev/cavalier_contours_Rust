use cavalier_contours::{core::traits::FuzzyEq, polyline::Polyline};
use static_aabb2d_index::AABB;

/// Fuzzy compare AABB values
pub fn aabb_fuzzy_eq_eps(a: &AABB<f64>, b: &AABB<f64>, eps: f64) -> bool {
    a.min_x.fuzzy_eq_eps(b.min_x, eps)
        && a.min_y.fuzzy_eq_eps(b.min_y, eps)
        && a.max_x.fuzzy_eq_eps(b.max_x, eps)
        && a.max_y.fuzzy_eq_eps(b.max_y, eps)
}

/// Holds a set of properties of a polyline for comparison in tests
#[derive(Debug, Copy, Clone)]
pub struct PlineProperties {
    pub vertex_count: usize,
    pub area: f64,
    pub path_length: f64,
    pub extents: AABB<f64>,
}

impl PlineProperties {
    pub fn new(
        vertex_count: usize,
        area: f64,
        path_length: f64,
        min_x: f64,
        min_y: f64,
        max_x: f64,
        max_y: f64,
    ) -> Self {
        Self {
            vertex_count,
            area,
            path_length,
            extents: AABB::new(min_x, min_y, max_x, max_y),
        }
    }

    pub fn from_pline(pline: &Polyline<f64>, invert_area: bool) -> Self {
        // remove redundant vertexes for consistent vertex counts
        let pline = pline.remove_redundant(1e-5);
        let area = {
            let a = pline.area();
            if invert_area {
                -a
            } else {
                a
            }
        };

        Self {
            vertex_count: pline.len(),
            area,
            path_length: pline.path_length(),
            extents: pline.extents().unwrap(),
        }
    }

    pub fn fuzzy_eq_eps(&self, other: &Self, eps: f64) -> bool {
        self.vertex_count == other.vertex_count
            && self.area.fuzzy_eq_eps(other.area, eps)
            && self.path_length.fuzzy_eq_eps(other.path_length, eps)
            && aabb_fuzzy_eq_eps(&self.extents, &other.extents, eps)
    }

    pub fn fuzzy_eq_eps_abs_a(&self, other: &Self, eps: f64) -> bool {
        self.vertex_count == other.vertex_count
            && self.area.abs().fuzzy_eq_eps(other.area.abs(), eps)
            && self.path_length.fuzzy_eq_eps(other.path_length, eps)
            && aabb_fuzzy_eq_eps(&self.extents, &other.extents, eps)
    }
}

pub fn create_property_set(polylines: &[Polyline<f64>], invert_area: bool) -> Vec<PlineProperties> {
    polylines
        .iter()
        .map(|pl| PlineProperties::from_pline(pl, invert_area))
        .collect()
}

pub fn property_sets_match(
    result_set: &[PlineProperties],
    expected_set: &[PlineProperties],
) -> bool {
    let mut sets_match = true;
    if result_set.len() != expected_set.len() {
        sets_match = false;
    } else {
        // using simple N^2 comparisons to compare property sets (sets are always relatively small,
        // e.g. N < 10)
        for properties_expected in expected_set {
            let match_count = result_set
                .iter()
                .filter(|properties_result| {
                    properties_expected.fuzzy_eq_eps(properties_result, 1e-5)
                })
                .count();

            if match_count != 1 {
                sets_match = false;
                break;
            }
        }
    }

    if !sets_match {
        eprintln!("result:\n{:?}", result_set);
        eprintln!("expected:\n{:?}", expected_set);
    }

    sets_match
}

pub fn property_sets_match_abs_a(
    result_set: &[PlineProperties],
    expected_set: &[PlineProperties],
) -> bool {
    let mut sets_match = true;
    if result_set.len() != expected_set.len() {
        sets_match = false;
    } else {
        // using simple N^2 comparisons to compare property sets (sets are always relatively small,
        // e.g. N < 10)
        for properties_expected in expected_set {
            let match_count = result_set
                .iter()
                .filter(|properties_result| {
                    properties_expected.fuzzy_eq_eps_abs_a(properties_result, 1e-5)
                })
                .count();

            if match_count != 1 {
                sets_match = false;
                break;
            }
        }
    }

    if !sets_match {
        eprintln!("result:\n{:?}", result_set);
        eprintln!("expected:\n{:?}", expected_set);
    }

    sets_match
}