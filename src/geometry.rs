use std::ops;

// Unfortunately, Rust doesn't yet allow square roots in constant contexts
pub const PHI: f64 = 1.618033988749895;
pub const PHI_INVERSE: f64 = PHI - 1.0; // == 1 / phi == 0.618033988749895
pub const DEG_TO_RAD: f64 = std::f64::consts::PI / 180.0;

// Since this relation is not transitive, it shouldn't be implemented via the Eq or ParialEq traits
pub trait Close {
    const TOLERANCE: f64 = 1e-5;
    fn is_close(&self, b: &Self) -> bool;
}

pub fn close<C: Close>(a: C, b: C) -> bool {
    a.is_close(&b)
}

impl Close for f64 {
    fn is_close(&self, b: &Self) -> bool {
        (self - b).abs() < Self::TOLERANCE
    }
}

macro_rules! assert_close {
    ($a:expr, $b:expr $(,)?) => {
        assert!(close($a, $b))
    };
}

pub trait Transform: Sized {
    fn rotate(&self, angle: f64) -> Self;
    fn mirror_x(&self) -> Self;
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
    fn is_close(&self, b: &Self) -> bool {
        self.0.is_close(&b.0) && self.1.is_close(&b.1)
    }
}

// @TODO: Add tests for `Transform` trait
impl Transform for Point {
    fn rotate(&self, angle: f64) -> Self {
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
pub enum RobinsonTriangleType {
    Small,
    Large,
}

#[derive(Clone)]
pub struct RobinsonTriangle {
    pub triangle_type: RobinsonTriangleType,
    pub a: Point,
    pub b: Point,
    pub c: Point,
}

impl RobinsonTriangle {
    fn infer_triangle_type(a: Point, b: Point, c: Point) -> RobinsonTriangleType {
        let (ab, bc, ca) = (
            Line(a, b).length(),
            Line(b, c).length(),
            Line(c, a).length(),
        );
        // Check that it is an isosceles triangle
        assert_close!(ab, bc);

        // Check that the sides are in a valid ratio and infer the triangle type from it
        if close(ca / ab, PHI) {
            RobinsonTriangleType::Large
        } else if close(ca / ab, PHI_INVERSE) {
            RobinsonTriangleType::Small
        } else {
            panic!()
        }
    }

    pub fn new(a: Point, b: Point, c: Point) -> Self {
        let triangle_type = RobinsonTriangle::infer_triangle_type(a, b, c);
        RobinsonTriangle { triangle_type, a, b, c }
    }

    pub fn from_base(
        a: Point,
        c: Point,
        triangle_type: RobinsonTriangleType,
        clockwise: bool,
    ) -> Self {
        let ratio = match triangle_type {
            RobinsonTriangleType::Small => PHI,
            RobinsonTriangleType::Large => PHI_INVERSE,
        };
        let median = (a + c) / 2.0;
        // Normalized direction vector from the median point to b
        let direction_to_b = {
            let a_to_c = c - a;
            let d = Point(a_to_c.1, -a_to_c.0).normalized();
            if clockwise { d } else { -d }
        };
        // Height of the resulting triangle
        let height = {
            let base_length = Line(a, c).length();
            let hypotenuse = base_length * ratio;
            (hypotenuse.powi(2) - (base_length / 2.0).powi(2)).sqrt()
        };
        let b = median + height * direction_to_b;

        // Just to make sure
        assert_eq!(triangle_type, RobinsonTriangle::infer_triangle_type(a, b, c));
        RobinsonTriangle { triangle_type, a, b, c }
    }

    /// Returns the median point of the triangle's base.
    pub fn base_median(&self) -> Point {
        (self.a + self.c) / 2.0
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

pub type Arc = (Point, Point, Point); // Start, center, end

pub struct Quadrilateral {
    pub a: Point,
    pub b: Point,
    pub c: Point,
    pub d: Point,
}

impl Quadrilateral {
    pub fn quadrilateral_type(&self) -> RobinsonTriangleType {
        if Line(self.a, self.c).length() > Line(self.b, self.d).length() {
            RobinsonTriangleType::Large
        } else {
            RobinsonTriangleType::Small
        }
    }

    pub fn arcs(&self) -> (Arc, Arc) {
        let first_arc = (
            Line(self.a, self.b).median(),
            self.a,
            Line(self.a, self.d).median(),
        );
        let second_arc = (
            Line(self.c, self.b).median(),
            self.c,
            Line(self.c, self.d).median(),
        );
        (first_arc, second_arc)
    }
}

// Useful for tests
#[cfg(test)]
pub fn random_point<R: rand::Rng>(rng: &mut R, min: f64, max: f64) -> Point {
    let x = rng.gen_range(min, max);
    let y = rng.gen_range(min, max);
    Point(x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_arithmetic() {
        let mut rng = rand::thread_rng();
        for _ in 0..10_000 {
            let a = random_point(&mut rng, 0.0, 1000.0);
            assert_close!(a, -(-a));
            assert_close!(a, a + Point::ZERO);
            assert_close!(a, a - Point::ZERO);
            assert_close!(a, 1.0 * a);
            assert_close!(a, a / 1.0);
            assert_close!(Point::ZERO, 0.0 * a);
            assert_close!(a.cross(a), 0.0);
            assert_close!(a.cross(-a), 0.0);

            let b = random_point(&mut rng, 0.0, 1000.0);
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
            let a = random_point(&mut rng, 0.0, 1000.0);
            assert_close!(Point::ZERO.distance_to(a.normalized()), 1.0);
            assert_close!(a.distance_to(a), 0.0);

            let b = random_point(&mut rng, 0.0, 1000.0);
            assert_close!(a.distance_to(b), b.distance_to(a));
        }
    }
}
