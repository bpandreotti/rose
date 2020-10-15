use crate::geometry::*;

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
    const RAD_TO_DEG: f64 = std::f64::consts::PI / 180.0;
    let a = Point(1.0, 0.0);
    let b = Point(f64::cos(36.0 * RAD_TO_DEG), -f64::sin(36.0 * RAD_TO_DEG));
    let c = Point(f64::cos(72.0 * RAD_TO_DEG), -f64::sin(72.0 * RAD_TO_DEG));
    let d = Point(-c.0, c.1);
    let e = Point(-b.0, b.1);
    let f = -a;
    let mut top_half = vec![
        RobinsonTriangle::new(a, Point::ZERO, b),
        RobinsonTriangle::new(c, Point::ZERO, b),
        RobinsonTriangle::new(c, Point::ZERO, d),
        RobinsonTriangle::new(e, Point::ZERO, d),
        RobinsonTriangle::new(e, Point::ZERO, f),
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

pub fn rhombus(rhombus_type: RobinsonTriangleType) -> Seed {
    let base_size = match rhombus_type {
        RobinsonTriangleType::Small => PHI_INVERSE,
        RobinsonTriangleType::Large => PHI,
    };
    let right = Point(base_size / 2.0, 0.0);
    let left = -right;
    Seed(vec![
        RobinsonTriangle::from_base(left, right, rhombus_type, true),
        RobinsonTriangle::from_base(left, right, rhombus_type, false)
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_side_length() {
        let all_seeds = [
            rose(),
            rhombus(RobinsonTriangleType::Small),
            rhombus(RobinsonTriangleType::Large),
        ];
        for s in &all_seeds {
            for t in &s.0 {
                assert_close!(Line(t.a, t.b).length(), 1.0);
                assert_close!(Line(t.c, t.b).length(), 1.0);
            }
        }
    }
}
