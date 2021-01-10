use crate::geometry::*;

#[derive(Clone)]
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
        RobinsonTriangle::new(p4, p1, Point::ZERO), // Inner petal
        RobinsonTriangle::new(p1, p4, p2),          // Outer petal
        RobinsonTriangle::new(p5, p4, p2),          // Leaf
        RobinsonTriangle::new(p5, p3, p2),          // Leaf
    ];

    // The rose can be divided in five equal sectors. Here we create the first one of these
    let mut first_sector = {
        let bottom_half = top_half.iter().map(Transform::mirror_y);
        let mut top_half = top_half.to_vec();
        top_half.extend(bottom_half);
        top_half
    };
    let sector_clone = first_sector.clone();

    // And then we create the remaining sectors by rotating the first one
    for i in 0..4 {
        let angle = (i + 1) as f64 * 72.0; // Angle goes from 72.0 to 288.0
        first_sector.extend(sector_clone.iter().map(|t| t.rotate(angle)));
    }

    Seed(first_sector)
}

pub fn rhombus(rhombus_type: TileType) -> Seed {
    let base_size = match rhombus_type {
        TileType::SmallRhombus => PHI_INVERSE,
        TileType::LargeRhombus => PHI,
        _ => todo!(),
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

    // First we create the first two triangles
    let mut triangles = vec![
        RobinsonTriangle::new(p1, Point::ZERO, p2),
        RobinsonTriangle::new(p3, Point::ZERO, p2),
    ];

    // And then create the remaining eight by rotating the first two
    for i in 0..4 {
        let angle = (i + 1) as f64 * 72.0; // Angle goes from 72.0 to 288.0
        triangles.push(triangles[0].rotate(angle));
        triangles.push(triangles[1].rotate(angle));
    }
    Seed(triangles)
}

#[cfg(test)]
pub fn get_all_seeds() -> [Seed; 4] {
    [
        rose(),
        rhombus(TileType::LargeRhombus),
        rhombus(TileType::SmallRhombus),
        pizza(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_side_length() {
        for s in &get_all_seeds() {
            for t in &s.0 {
                assert_close!(Line(t.a, t.b).length(), 1.0);
                assert_close!(Line(t.c, t.b).length(), 1.0);
            }
        }
    }
}
