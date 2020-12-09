use crate::geometry::*;

const DEG_TO_RAD: f64 = std::f64::consts::PI / 180.0;

// @TODO: Maybe make this a method of `Point`?
pub fn rotate_point(Point(x, y): Point, angle: f64) -> Point {
    let cos = f64::cos(DEG_TO_RAD * angle);
    let sin = f64::sin(DEG_TO_RAD * angle);
    Point(x * cos - y * sin, x * sin + y * cos)
}

pub struct Seed(Vec<RobinsonTriangle>);

impl Seed {
    pub fn transform(self, center: Point, scale: f64) -> Vec<RobinsonTriangle> {
        self.0
            .iter()
            .map(|t| {
                let RobinsonTriangle { a, b, c, .. } = *t;
                RobinsonTriangle::new(scale * a + center, scale * b + center, scale * c + center)
            })
            .collect()
    }
}

pub fn rose() -> Seed {
    let p1 = Point(1.0, 0.0);
    let p2 = Point(PHI, 0.0);
    let p3 = p2 + rotate_point(p1, 36.0);
    let p4 = rotate_point(p2, 36.0);
    let p5 = rotate_point(p2 + p1, 36.0);

    let top_half = [
        RobinsonTriangle::new(p4, p1, Point::ZERO),
        RobinsonTriangle::new(p1, p4, p2),
        RobinsonTriangle::new(p5, p4, p2),
        RobinsonTriangle::new(p5, p3, p2),
    ];

    // @TODO: Make this a general function or method
    fn flip_y(RobinsonTriangle { a, b, c, .. }: &RobinsonTriangle) -> RobinsonTriangle {
        RobinsonTriangle::new(
            Point(a.0, -a.1),
            Point(b.0, -b.1),
            Point(c.0, -c.1),
        )
    }
    let bottom_half = top_half.iter().map(flip_y);
    let mut first_sector = top_half.to_vec();
    first_sector.extend(bottom_half);

    fn rotate_triangles(triangles: &[RobinsonTriangle], angle: f64) -> Vec<RobinsonTriangle> {
        let mut result = Vec::with_capacity(triangles.len());
        for t in triangles {
            let r = RobinsonTriangle::new(
                rotate_point(t.a, angle),
                rotate_point(t.b, angle),
                rotate_point(t.c, angle),
            );
            result.push(r)
        }
        result
    }

    let other_slices = vec![
        rotate_triangles(&first_sector, 72.0),
        rotate_triangles(&first_sector, 144.0),
        rotate_triangles(&first_sector, 216.0),
        rotate_triangles(&first_sector, 288.0),
    ];
    for s in other_slices.into_iter() {
        first_sector.extend(s.into_iter());
    }

    Seed(first_sector)
}

pub fn rhombus(rhombus_type: RobinsonTriangleType) -> Seed {
    let base_size = match rhombus_type {
        RobinsonTriangleType::Small => PHI_INVERSE,
        RobinsonTriangleType::Large => PHI,
    };
    let right = Point(base_size / 2.0, 0.0);
    let left = -right;
    Seed(vec![
        RobinsonTriangle::from_base(left, right, rhombus_type, true),
        RobinsonTriangle::from_base(left, right, rhombus_type, false),
    ])
}

pub fn pizza() -> Seed {
    let p1 = Point(1.0, 0.0);
    let p2 = Point(f64::cos(36.0 * DEG_TO_RAD), -f64::sin(36.0 * DEG_TO_RAD));
    let p3 = Point(f64::cos(72.0 * DEG_TO_RAD), -f64::sin(72.0 * DEG_TO_RAD));
    let p4 = Point(-p3.0, p3.1);
    let p5 = Point(-p2.0, p2.1);
    let p6 = -p1;
    let mut top_half = vec![
        RobinsonTriangle::new(p1, Point::ZERO, p2),
        RobinsonTriangle::new(p3, Point::ZERO, p2),
        RobinsonTriangle::new(p3, Point::ZERO, p4),
        RobinsonTriangle::new(p5, Point::ZERO, p4),
        RobinsonTriangle::new(p5, Point::ZERO, p6),
    ];
    let mut bottom_half = top_half
        .iter()
        .map(|t| {
            let RobinsonTriangle { a, b, c, .. } = t;
            RobinsonTriangle::new(Point(a.0, -a.1), *b, Point(c.0, -c.1))
        })
        .collect::<Vec<_>>();
    top_half.append(&mut bottom_half);
    Seed(top_half)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_side_length() {
        let all_seeds = [
            rose(),
            rhombus(RobinsonTriangleType::Large),
            rhombus(RobinsonTriangleType::Small),
            pizza(),
        ];
        for s in &all_seeds {
            for t in &s.0 {
                assert_close!(Line(t.a, t.b).length(), 1.0);
                assert_close!(Line(t.c, t.b).length(), 1.0);
            }
        }
    }
}
