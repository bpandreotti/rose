use crate::geometry::*;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

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

pub fn merge_pairs_hashing(triangles: Vec<RobinsonTriangle>) -> Vec<Quadrilateral> {
    // The basic idea of this algorithm is to use a hash map indexed by the triangles bases' 
    // medians, and to iterate through the triangles vector inserting them into the map. If there is
    // already a triangle with that same hash, they have the same base median, and can be merged.
    // This would be O(n). Since the coordinates are stored as floating point numbers, we would have
    // a problem if two triangles have base medians that are very close (closer than the `Close`
    // trait tolerance) but still not exactly the same. In this case, their base medians would have
    // completely different hashes, and the triangles would not be merged. Because of that, before
    // we populate the hash map, we have to divide the points into discrete buckets and use these
    // buckets for hashing, so that two base medians that are close will very likely be in the same
    // bucket, and therefore will have the same hash. This introduces the possibilty for two errors:
    // collisions and misses. If two base medians aren't close, but happen to land on the same
    // bucket, we have a collision. If two base medians are close, but happen to land on the border
    // between two buckets, such that they fall on different buckets, we would have a miss. Misses
    // would leave some triangles remaining on the hash map, and we would have to go through it and
    // try to find their pairs in the end. This is easier to do than trying to resolve collisions,
    // so we choose the buckets accordingly.

    let side_length = match triangles.get(0) {
        Some(t) => Line(t.a, t.b).length(),
        None => return vec![], // In case of empty triangles vector
    };
    let scaling_factor = 100.0 / side_length;

    // This is how we divide the points into buckets. We couuld just floor each point to an integer,
    // but that would result in very large buckets, and make collisions very likely. Instead, we
    // scale the points first, so that when we floor them to integers each bucket will be relatively
    // smaller, avoiding collisions. If we make the scaling factor too large, the buckets will be
    // too small, leading to more misses. This is a balancing act.
    let round_point_to_bucket = |Point(x, y)| -> (i32, i32) {
        ((x * scaling_factor) as i32, (y * scaling_factor) as i32)
    };

    // We insert the indices instead of the actual triangles to save memory.
    let mut map = HashMap::<(i32, i32), usize>::with_capacity(triangles.len());
    let mut result = Vec::with_capacity(triangles.len() / 2);
    for (i, t) in triangles.iter().enumerate() {
        let rounded = round_point_to_bucket(t.base_median());
        match map.entry(rounded) {
            Entry::Occupied(o) => {
                // If there is a triangle in this bucket, we remove it from the hash map, and merge
                // the triangles. We have to remove it to make fixing misses easier later.
                let (_, other) = o.remove_entry();
                let other = &triangles[other];
                assert_close!(other.base_median(), t.base_median());
                result.push(Quadrilateral {
                    a: t.a,
                    b: t.b,
                    c: t.c,
                    d: other.b,
                });
            }
            Entry::Vacant(v) => {
                // If there is no triangle in this bucket, we just insert the current triangle.
                v.insert(i);
            }
        }
    }

    // Since we removed all triangles that were merged, the only triangles left in the map are
    // either misses, or triangles that actually have no pair and won't be rendered. A lazy way to
    // find the missed pairs is to just use the other O(n log n) algorithm. These misses would be
    // very rare if the triangles were randomly positioned, but the way the tiling is generated make
    // it so triangles are generated with round integer coordinates, resulting in misses pretty
    // consistently. Still, they make up a small amount of the total triangles, so we don't need to
    // worry so much about optimizing this step.
    let remaining = map.values().map(|i| triangles[*i].clone()).collect::<Vec<_>>();
    result.extend(merge_pairs(remaining));
    result
}

fn decompose(rt: RobinsonTriangle) -> Vec<RobinsonTriangle> {
    let RobinsonTriangle { triangle_type, a, b, c } = rt;
    match triangle_type {
        TileType::SmallRhombus => {
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
        TileType::LargeRhombus => {
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
        _ => todo!(),
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
                    TileType::LargeRhombus
                } else {
                    TileType::SmallRhombus
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
    fn test_merge_pairs_hashing() {
        for seed in crate::seeds::get_all_seeds().iter() {
            let triangles = generate_tiling(seed.clone().transform(Point(500.0, 500.0), 1000.0), 3);
            let mut expected = merge_pairs(triangles.clone());
            let mut got = merge_pairs_hashing(triangles);

            // Make sure we didn't miss any triangles
            assert_eq!(expected.len(), got.len());

            let quad_center = |&Quadrilateral { a, b, c, d }: &_| (a + b + c + d) / 4.0;
            let quad_compare = |q1: &_, q2: &_| Point::compare(quad_center(q1), quad_center(q2));

            expected.sort_by(quad_compare);
            got.sort_by(quad_compare);
            for (e, g) in expected.into_iter().zip(got) {
                assert_close!(quad_center(&e), quad_center(&g))
            }
        }
    }


    /// This tests many combinations of seeds, scales, and generations to make sure that
    /// `merge_pairs_hashing` will never result in a collision. This test can be pretty slow
    #[test]
    fn test_merge_pairs_hashing_collision() {
        for seed in crate::seeds::get_all_seeds().iter() {
            for size in 1..5 {
                let scale = size as f64 * 200.0;
                let mut triangles = seed.clone().transform(Point(scale / 2.0, scale / 2.0), scale);
                for _ in 0..8 {
                    triangles = triangles.into_iter().flat_map(decompose).collect();
                    merge_pairs_hashing(triangles.clone());
                }
            }
        }
    }

    #[test]
    fn test_matching_rules() {
        // Create two rhombuses, a small one centered at x=1000, and a large one next to it,
        // centered at x=4000. They are scaled appropriately.
        let seed = {
            let mut small = crate::seeds::rhombus(TileType::SmallRhombus)
                .transform(Point(1000.0, 2000.0), 1000.0);
            let large = crate::seeds::rhombus(TileType::LargeRhombus)
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
                    TileType::SmallRhombus => vec![
                        (EdgeType::Type1, Line(b, a)),
                        (EdgeType::Type2, Line(b, c)),
                        (EdgeType::SmallBase, Line(a, c)),
                    ],
                    TileType::LargeRhombus => vec![
                        (EdgeType::Type1, Line(a, b)),
                        (EdgeType::Type2, Line(b, c)),
                        (EdgeType::LargeBase, Line(a, c)),
                    ],
                    _ => todo!(),
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
