use crate::geometry::*;

pub fn generate_tiling(seed: Vec<RobinsonTriangle>, num_generations: u64) -> Vec<RobinsonTriangle> {
    let mut triangles = seed;
    for _ in 0..num_generations {
        triangles = triangles.into_iter().flat_map(decompose).collect();
    }
    triangles
}

pub fn merge_pairs(mut triangles: Vec<RobinsonTriangle>) -> Vec<Quadrilateral> {
    // We could compare every triangle with every other triangle and check if their bases are
    // adjacent, but that would be O(n^2). Instead, we sort them by the position of their bases'
    // medians, so that two triangles with adjacent bases would be next to each other on the vector.
    // This step is O(n log n). After that it's just an O(n) pass through the vector to find the
    // pairs.
    triangles.sort_by(|a, b| Point::compare(a.base_median(), b.base_median()));
    triangles
        .windows(2)
        .filter_map(|ts| {
            let (current, next) = (&ts[0], &ts[1]);
            if close(current.base_median(), next.base_median()) {
                // If the base medians are the same, then the base vertices should also be the same
                assert!(close(current.a, next.a) || close(current.a, next.c));
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

fn decompose(rt: RobinsonTriangle) -> Vec<RobinsonTriangle> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::random_point;
    use rand::Rng;

    #[test]
    fn test_merge_pairs() {
        let mut rng = rand::thread_rng();
        let triangles = (0..10_000)
            .flat_map(|_| {
                let p = random_point(&mut rng, -1000.0, 1000.0);
                let q = random_point(&mut rng, -1000.0, 1000.0);
                let triangle_type = if rng.gen() {
                    RobinsonTriangleType::Large
                } else {
                    RobinsonTriangleType::Small
                };
                vec![
                    RobinsonTriangle::from_base(p, q, triangle_type, true),
                    RobinsonTriangle::from_base(p, q, triangle_type, false),
                ]
            })
            .collect::<Vec<_>>();
        let original_len = triangles.len();
        let rhombs = merge_pairs(triangles);
        assert_eq!(original_len, rhombs.len() * 2)
    }

    #[test]
    fn test_matching_rules() {
        // Create two rhombuses, a small one centered at x=1000, and a large one next to it,
        // centered at x=4000. They are scaled appropriately.
        let seed = {
            let mut small = crate::seeds::rhombus(RobinsonTriangleType::Small)
                .transform(Point(1000.0, 2000.0), 1000.0);
            let large = crate::seeds::rhombus(RobinsonTriangleType::Large)
                .transform(Point(4000.0, 2000.0), 1000.0);
            small.extend(large);
            small
        };

        // Decompose them for eight generations
        let triangles = generate_tiling(seed, 8);

        #[derive(Debug, PartialEq)]
        enum EdgeType {
            Type1,     // Goes from B to A in a small triangle or A to B in a large one
            Type2,     // Goes from B to C in a small or large triangle
            SmallBase, // Goes from A to C in a small triangle
            LargeBase, // Goes from A to C in a large triangle
        }

        // Split up the triangles into their composing edges
        let mut edges: Vec<(EdgeType, Line)> = triangles
            .into_iter()
            .flat_map(|t| {
                let RobinsonTriangle { triangle_type, a, b, c } = t;
                match triangle_type {
                    RobinsonTriangleType::Small => vec![
                        (EdgeType::Type1, Line(b, a)),
                        (EdgeType::Type2, Line(b, c)),
                        (EdgeType::SmallBase, Line(a, c)),
                    ],
                    RobinsonTriangleType::Large => vec![
                        (EdgeType::Type1, Line(a, b)),
                        (EdgeType::Type2, Line(b, c)),
                        (EdgeType::LargeBase, Line(a, c)),
                    ],
                }
            })
            .collect();

        // We sort here for reasons similar to those in `merge_pairs`
        edges.sort_by(|a, b| Point::compare(a.1.median(), b.1.median()));

        // Are there any three edges that have the same median?
        for es in edges.windows(3) {
            if let [(_, first), (_, second), (_, third)] = es {
                let first_median = first.median();
                let second_median = second.median();
                let third_median = third.median();
                assert!(
                    !(close(first_median, second_median) && close(second_median, third_median))
                );
            } else {
                unreachable!()
            }
        }

        // Are there any adjacent edges that have different types or orientations?
        for es in edges.windows(2) {
            if let [(current_type, current_line), (next_type, next_line)] = es {
                let (c_median, n_median) = (current_line.median(), next_line.median());
                if close(c_median, n_median) {
                    assert_eq!(current_type, next_type);
                    assert_close!(current_line.0, next_line.0);
                    assert_close!(current_line.1, next_line.1);
                }
            } else {
                unreachable!()
            }
        }
    }
}
