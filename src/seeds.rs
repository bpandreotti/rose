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
    let p1 = Point(1.0, 0.0);
    let p2 = Point(PHI, 0.0);
    let p3 = p2 + p1.rotate(36.0);
    let p4 = p2.rotate(36.0);
    let p5 = (p2 + p1).rotate(36.0);

    let top_half = [
        RobinsonTriangle::new(p4, p1, Point::ZERO),
        RobinsonTriangle::new(p1, p4, p2),
        RobinsonTriangle::new(p5, p4, p2),
        RobinsonTriangle::new(p5, p3, p2),
    ];

    let mut first_sector = {
        let bottom_half = top_half.iter().map(Transform::mirror_y);
        let mut top_half = top_half.to_vec();
        top_half.extend(bottom_half);
        top_half
    };
    let sector_clone = first_sector.clone();
    for angle in &[72.0, 144.0, 216.0, 288.0] {
        first_sector.extend(sector_clone.iter().map(|t| t.rotate(*angle)));
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
    let p2 = p1.rotate(36.0);
    let p3 = p1.rotate(72.0);

    let mut triangles = vec![
        RobinsonTriangle::new(p1, Point::ZERO, p2),
        RobinsonTriangle::new(p3, Point::ZERO, p2),
    ];
    for i in 0..4 {
        // Angle goes from 72.0 to 288.0
        let angle = (i + 1) as f64 * 72.0;
        triangles.push(triangles[0].rotate(angle));
        triangles.push(triangles[1].rotate(angle));
    }
    Seed(triangles)
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
