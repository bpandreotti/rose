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
            // The large triangle will be divided in three: two large ADE and CEB triangles, and a
            // small BED triangle.
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
