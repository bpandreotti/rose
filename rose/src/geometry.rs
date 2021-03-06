use std::ops;

// Unfortunately, Rust doesn't yet allow square roots in constant contexts
pub const PHI: f64 = 1.618033988749895;
pub const PHI_INVERSE: f64 = PHI - 1.0; // == 1 / phi == 0.618033988749895
pub const DEG_TO_RAD: f64 = std::f64::consts::PI / 180.0;

// Since this relation is not transitive, it shouldn't be implemented via the Eq or ParialEq traits
pub trait Close {
    const TOLERANCE: f64 = 1e-5;
    fn is_close(&self, other: &Self) -> bool;
}

pub fn close<C: Close>(a: C, b: C) -> bool {
    a.is_close(&b)
}

impl Close for f64 {
    fn is_close(&self, other: &Self) -> bool {
        (self - other).abs() < Self::TOLERANCE
    }
}

macro_rules! assert_close {
    ($a:expr, $b:expr $(,)?) => {
        assert!(close($a, $b))
    };
}

/// A trait that represents common transformations on geometric objects
pub trait Transform: Sized {
    /// Rotate the object aroud the origin by a ginve angle, in degrees.
    fn rotate(&self, angle: f64) -> Self;

    /// Mirror the object across the x axis
    fn mirror_x(&self) -> Self;

    /// Mirror the object across the y axis
    fn mirror_y(&self) -> Self;
}

#[derive(Debug, Copy, Clone)]
pub struct Point(pub f64, pub f64);

impl Point {
    pub const ZERO: Point = Point(0.0, 0.0);

    pub fn distance_to(&self, point: Point) -> f64 {
        ((point.0 - self.0).powi(2) + (point.1 - self.1).powi(2)).sqrt()
    }

    pub fn normalized(&self) -> Self {
        *self / Point::ZERO.distance_to(*self)
    }

    pub fn cross(&self, other: Point) -> f64 {
        self.0 * other.1 - self.1 * other.0
    }

    /// Compares two points by their x coordinate. If their x coordinates are close (as defined by
    /// the `Close` trait), compare by their y coordinate.
    pub fn compare(a: Point, b: Point) -> std::cmp::Ordering {
        use std::cmp::Ordering;
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
    }
}

impl Close for Point {
    fn is_close(&self, other: &Self) -> bool {
        // Two points are close if both their coordinates are close
        self.0.is_close(&other.0) && self.1.is_close(&other.1)
    }
}

impl Transform for Point {
    // A rotation by a positive angle is a clockwise rotation in SVG coordinates, or an
    // anti-clockwise rotation in traditional cartesian coordinates. For example:
    //   Point(1.0, 0.0).rotate(90.0) == Point(0.0, 1.0)
    fn rotate(&self, angle: f64) -> Self {
        // See https://en.wikipedia.org/wiki/Rotation_matrix
        let cos = f64::cos(DEG_TO_RAD * angle);
        let sin = f64::sin(DEG_TO_RAD * angle);
        Point(self.0 * cos - self.1 * sin, self.0 * sin + self.1 * cos)
    }

    fn mirror_x(&self) -> Self {
        Point(-self.0, self.1)
    }

    fn mirror_y(&self) -> Self {
        Point(self.0, -self.1)
    }
}

impl ops::Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl ops::Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Point(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl ops::Neg for Point {
    type Output = Self;
    fn neg(self) -> Self {
        Point::ZERO - self
    }
}

impl ops::Mul<Point> for f64 {
    type Output = Point;
    fn mul(self, rhs: Point) -> Self::Output {
        Point(self * rhs.0, self * rhs.1)
    }
}

impl ops::Div<f64> for Point {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)] // Trust me, clippy
    fn div(self, rhs: f64) -> Self {
        rhs.recip() * self
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Line(pub Point, pub Point);

impl Line {
    pub fn length(&self) -> f64 {
        self.0.distance_to(self.1)
    }

    pub fn median(&self) -> Point {
        (self.0 + self.1) / 2.0
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TileType {
    SmallRhombus,
    LargeRhombus,
    Kite,
    Dart,
}

impl TileType {
    pub fn base_to_side_ratio(self) -> f64 {
        match self {
            TileType::SmallRhombus => PHI_INVERSE,
            TileType::LargeRhombus => PHI,
            // For the Kite and Dart tiles, we consider the ratio of the base to the largest side
            TileType::Kite => 1.0,
            TileType::Dart => PHI_INVERSE,
        }
    }
}

#[derive(Clone)]
pub struct RobinsonTriangle {
    pub triangle_type: TileType,
    pub a: Point,
    pub b: Point,
    pub c: Point,
}

impl RobinsonTriangle {
    pub fn infer_triangle_type(a: Point, b: Point, c: Point) -> TileType {
        let (ab, bc, ca) = (
            Line(a, b).length(),
            Line(b, c).length(),
            Line(c, a).length(),
        );
        let ratio = ca / ab;

        // For the rhombus triangles, we must make sure that the triangle is isoceles
        if close(ab, bc) && close(ratio, TileType::LargeRhombus.base_to_side_ratio()) {
            TileType::LargeRhombus
        } else if close(ab, bc) && close(ratio, TileType::SmallRhombus.base_to_side_ratio()) {
            TileType::SmallRhombus
        } else if close(ratio, TileType::Kite.base_to_side_ratio()) {
            TileType::Kite
        } else if close(ratio, TileType::Dart.base_to_side_ratio()) {
            TileType::Dart
        } else {
            panic!("Triangle sides are of invalid ratio")
        }
    }

    pub fn new(a: Point, b: Point, c: Point) -> Self {
        let triangle_type = RobinsonTriangle::infer_triangle_type(a, b, c);
        RobinsonTriangle {
            triangle_type,
            a,
            b,
            c,
        }
    }

    pub fn from_base(a: Point, c: Point, triangle_type: TileType, right_handed: bool) -> Self {
        let rotation = {
            // The angle at the a vertex
            let a_angle = match triangle_type {
                TileType::SmallRhombus => 72.0,
                _ => 36.0,
            };

            // If the triangle is right-handed -- that is, if the a to b to c path makes a right
            // turn in SVG coordinates -- the direction from a to b is the direction from a to c
            // rotated anti-clockwise. Of course, if the triangle is left-handed, it's the opposite.
            if right_handed {
                -a_angle
            } else {
                a_angle
            }
        };

        // Normalized direction vector from a to b
        let direction_to_b = (c - a).rotate(rotation).normalized();

        // The length of ab is simply the base length divided by the base to side ratio, since in
        // Kite and Dart triangles the largest side is ab
        let ab_length = Line(a, c).length() / triangle_type.base_to_side_ratio();
        let b = a + ab_length * direction_to_b;

        RobinsonTriangle {
            triangle_type,
            a,
            b,
            c,
        }
    }

    /// Returns the median point of the triangle's base.
    pub fn base_median(&self) -> Point {
        (self.a + self.c) / 2.0
    }
}

impl Close for RobinsonTriangle {
    fn is_close(&self, other: &Self) -> bool {
        close(self.a, other.a) && close(self.b, other.b) && close(self.c, other.c)
    }
}

impl Transform for RobinsonTriangle {
    fn rotate(&self, angle: f64) -> Self {
        RobinsonTriangle::new(
            self.a.rotate(angle),
            self.b.rotate(angle),
            self.c.rotate(angle),
        )
    }

    fn mirror_x(&self) -> Self {
        RobinsonTriangle::new(self.a.mirror_x(), self.b.mirror_x(), self.c.mirror_x())
    }

    fn mirror_y(&self) -> Self {
        RobinsonTriangle::new(self.a.mirror_y(), self.b.mirror_y(), self.c.mirror_y())
    }
}

pub type Arc = (Point, Point, Point, bool); // Start, center, end, large angle flag

pub struct Quadrilateral {
    pub a: Point,
    pub b: Point,
    pub c: Point,
    pub d: Point,
}

// Useful for tests
#[cfg(test)]
pub fn random_point<R: rand::Rng>(rng: &mut R, min: f64, max: f64) -> Point {
    let x = rng.gen_range(min..max);
    let y = rng.gen_range(min..max);
    Point(x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_arithmetic() {
        let mut rng = rand::thread_rng();
        for _ in 0..10_000 {
            let a = random_point(&mut rng, -1000.0, 1000.0);
            assert_close!(a, -(-a));
            assert_close!(a, a + Point::ZERO);
            assert_close!(a, a - Point::ZERO);
            assert_close!(a, 1.0 * a);
            assert_close!(a, a / 1.0);
            assert_close!(Point::ZERO, 0.0 * a);
            assert_close!(a.cross(a), 0.0);
            assert_close!(a.cross(-a), 0.0);

            let b = random_point(&mut rng, -1000.0, 1000.0);
            assert_close!(a + b, b + a);
            assert_close!(a - b, -(b - a));
            assert_close!(a.cross(b), -(b.cross(a)));
            assert_close!(a.cross(b), -((-a).cross(b)));
            assert_close!(a.cross(b), (-a).cross(-b));
        }
    }

    #[test]
    fn test_point_distance() {
        let mut rng = rand::thread_rng();
        for _ in 0..10_000 {
            let a = random_point(&mut rng, -1000.0, 1000.0);
            assert_close!(Point::ZERO.distance_to(a.normalized()), 1.0);
            assert_close!(a.distance_to(a), 0.0);

            let b = random_point(&mut rng, -1000.0, 1000.0);
            assert_close!(a.distance_to(b), b.distance_to(a));
        }
    }

    fn run_transform_tests<T: Transform + Close + Clone, R: rand::Rng>(obj: &T, rng: &mut R) {
        assert_close!(obj.mirror_x().mirror_x(), obj.clone());
        assert_close!(obj.mirror_y().mirror_y(), obj.clone());
        assert_close!(obj.mirror_x().mirror_y(), obj.mirror_y().mirror_x());
        assert_close!(obj.mirror_x().mirror_y(), obj.rotate(180.0));
        assert_close!(obj.rotate(0.0), obj.clone());
        assert_close!(obj.rotate(360.0), obj.clone());
        let x = rng.gen_range(0.0..360.0);
        assert_close!(obj.rotate(x).rotate(x), obj.rotate(2.0 * x));
        assert_close!(obj.rotate(x).rotate(-x), obj.clone());
    }

    #[test]
    fn test_point_transform() {
        let mut rng = rand::thread_rng();
        for _ in 0..10_000 {
            let a = random_point(&mut rng, -1000.0, 1000.0);
            run_transform_tests(&a, &mut rng);
        }
    }

    #[test]
    fn test_triangle_transform() {
        let mut rng = rand::thread_rng();
        for _ in 0..10_000 {
            let a = random_point(&mut rng, -1000.0, 1000.0);
            let c = random_point(&mut rng, -1000.0, 1000.0);
            use TileType::*;
            let types = [SmallRhombus, LargeRhombus, Kite, Dart];
            for &triangle_type in &types {
                run_transform_tests(
                    &RobinsonTriangle::from_base(a, c, triangle_type, false),
                    &mut rng,
                );
                run_transform_tests(
                    &RobinsonTriangle::from_base(a, c, triangle_type, true),
                    &mut rng,
                );
            }
        }
    }

    #[test]
    fn test_triangle_from_base() {
        let mut rng = rand::thread_rng();
        for _ in 0..10_000 {
            let a = random_point(&mut rng, -1000.0, 1000.0);
            let c = random_point(&mut rng, -1000.0, 1000.0);
            use TileType::*;
            let types = [SmallRhombus, LargeRhombus, Kite, Dart];
            for &triangle_type in &types {
                assert_eq!(
                    RobinsonTriangle::from_base(a, c, triangle_type, false).triangle_type,
                    triangle_type
                );
                assert_eq!(
                    RobinsonTriangle::from_base(a, c, triangle_type, true).triangle_type,
                    triangle_type
                );
            }
        }
    }
}
