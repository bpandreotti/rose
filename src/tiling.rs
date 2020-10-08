use crate::geometry::*;

pub fn inflate_all(triangles: Vec<RobinsonTriangle>) -> Vec<RobinsonTriangle> {
    triangles.into_iter().flat_map(inflate).collect()
}

pub fn inflate(rt: RobinsonTriangle) -> Vec<RobinsonTriangle> {
    let RobinsonTriangle { triangle_type, a, b, c } = rt;
    match triangle_type {
        RobinsonTriangleType::Small => {
            // The small triangle will be divided in two: a small DCA triangle and a large CDB
            // triangle.
            //        B
            //        /\
            //       /  \
            //      /    \
            //   D *      \
            //    /        \
            // A /__________\ C
            // It can be shown that:
            //   the length of BD == (the length of BA) / phi
            let d = b + (a - b) / PHI;
            vec![
                RobinsonTriangle::new(d, c, a),
                RobinsonTriangle::new(c, d, b),
            ]
        }
        RobinsonTriangleType::Large => {
            // The large triangle will be divided in three: two large EDA and CEB triangles, and a
            // small DEB triangle.
            // ce = ca / phi^2
            // bd = ba / phi^2
            //     A
            //     |\
            //     | \
            //     |  \
            //     |   * D
            //     |    \
            //     |     \ B
            //   E *     /
            //     |    /
            //     |   /
            //     |  /
            //     | /
            //     |/
            //     C
            // It can be shown that:
            //   the length of AD == (the length of AB) / phi
            //   the length of AE == (the length of AC) / phi
            let d = a + (b - a) / PHI;
            let e = a + (c - a) / PHI;
            vec![
                RobinsonTriangle::new(e, d, a),
                RobinsonTriangle::new(c, e, b),
                RobinsonTriangle::new(d, e, b),
            ]
        }
    }
}

pub fn merge_pairs(mut triangles: Vec<RobinsonTriangle>) -> Vec<Quadrilateral> {
    // We could compare every triangle with every other triangle and check if their bases are
    // adjacent, but that would be O(n^2). Instead, we sort them by the position of their bases'
    // medians, so that two triangles with adjacent bases would be next to each other on the vector.
    // This step is O(n log n). After that it's just an O(n) pass through the vector to find the
    // pairs.
    triangles.sort_by(|a, b| {
        use std::cmp::Ordering;
        let (a, b) = (a.base_median(), b.base_median());
        if close(a.0, b.0) {
            if close(a.1, b.1) {
                Ordering::Equal
            } else if a.1 > b.1 {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        } else if a.0 > b.0 {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    });
    triangles
        .windows(2)
        .filter_map(|ts| {
            let (current, next) = (&ts[0], &ts[1]);
            if current.base_median().close(next.base_median()) {
                // If the base medians are the same, then the base vertices should also be the same
                assert!(current.a.close(next.a) || current.a.close(next.c));
                Some(Quadrilateral {
                    a: current.a,
                    b: current.b,
                    c: current.c,
                    d: next.b,
                })
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    fn random_point<R: Rng>(rng: &mut R, min: f64, max: f64) -> Point {
        let x = rng.gen_range(min, max);
        let y = rng.gen_range(min, max);
        Point(x, y)
    }

    #[test]
    fn test_merge_pairs() {
        let mut rng = rand::thread_rng();
        let triangles = (0..10_000).flat_map(|_| {
            let p = random_point(&mut rng, 0.0, 1000.0);
            let q = random_point(&mut rng, 0.0, 1000.0);
            let triangle_type = if rng.gen() {
                RobinsonTriangleType::Large
            } else {
                RobinsonTriangleType::Small
            };
            vec![
                RobinsonTriangle::from_base(p, q, triangle_type, true),
                RobinsonTriangle::from_base(p, q, triangle_type, false)
            ]
        }).collect::<Vec<_>>();
        let original_len = triangles.len();
        let rhombs = merge_pairs(triangles);
        assert_eq!(original_len, rhombs.len() * 2)
    }
}
