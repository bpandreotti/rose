use std::ops;

// Unfortunately, Rust doesn't yet allow square roots in constant contexts
const PHI: f64 = 1.618033988749895;
const PHI_INVERSE: f64 = PHI - 1.0; // == 1 / phi == 0.618033988749895

fn close(a: f64, b: f64) -> bool {
    const TOLERANCE: f64 = 1e-5;
    (a - b).abs() < TOLERANCE
}

macro_rules! assert_close {
    ($a:expr, $b:expr $(,)?) => {
        assert!(close($a, $b))
    };
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
}

pub fn is_clockwise_turn(Line(a, b): Line, c: Point) -> bool {
    // Clockwise with respect to the coordinate system of the svg, that is, positive Y is down
    (b - a).cross(c - b) > 0.0
}

pub enum RobinsonTriangleType {
    Small,
    Large,
}

pub struct RobinsonTriangle {
    triangle_type: RobinsonTriangleType,
    a: Point,
    b: Point,
    c: Point,
}

impl RobinsonTriangle {
    // This function could use a better name
    fn check_invariants(a: Point, b: Point, c: Point) -> RobinsonTriangleType {
        // Check that the vertices are in the right order
        assert!(is_clockwise_turn(Line(a, b), c));
        
        let (ab, bc, ca) = (Line(a, b).length(), Line(b, c).length(), Line(c, a).length());
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
        let triangle_type = RobinsonTriangle::check_invariants(a, b, c);
        RobinsonTriangle { triangle_type, a, b, c }
    }

    pub fn from_base(a: Point, c: Point, triangle_type: RobinsonTriangleType) -> Self {
        let ratio = match triangle_type {
            RobinsonTriangleType::Small => PHI,
            RobinsonTriangleType::Large => PHI_INVERSE,
        };
        let median = (a + c) / 2.0;
        // Normalized direction vector from the median point to b
        let direction_to_b = {
            let b_direction = c - a;
            Point(b_direction.1, -b_direction.0).normalized()
        };
        // Height of the resulting triangle
        let height = {
            let base_length = Line(a, c).length();
            let hypotenuse = base_length * ratio;
            (hypotenuse.powi(2) - (base_length / 2.0).powi(2)).sqrt()
        };
        let b = median + height * direction_to_b;
        RobinsonTriangle::check_invariants(a, b, c); // Just to make sure
        RobinsonTriangle { triangle_type, a, b, c }
    }

    pub fn lines(&self) -> [Line; 3] {
        [Line(self.a, self.b), Line(self.b, self.c), Line(self.c, self.a)]
    }
}
