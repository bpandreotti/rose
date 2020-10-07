use std::ops;

// Unfortunately, Rust doesn't yet allow square roots in constant contexts.
const PHI: f64 = 1.618033988749895;
const PHI_INVERSE: f64 = PHI - 1.0; // == 1 / phi == 0.618033988749895

fn close(a: f64, b: f64) -> bool {
    const TOLERANCE: f64 = 1e-5;
    (a - b).abs() < TOLERANCE
}

#[derive(Debug, Copy, Clone)]
pub struct Point(pub f64, pub f64);

impl Point {
    const ZERO: Point = Point(0.0, 0.0);

    pub fn distance_to(&self, point: Point) -> f64 {
        ((point.0 - self.0).powi(2) + (point.1 - self.1).powi(2)).sqrt()
    }

    pub fn normalized(&self) -> Self {
        *self / Point::ZERO.distance_to(*self)
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
    pub fn new(a: Point, b: Point, c: Point) -> Self {
        let (ab, bc, ca) = (Line(a, b).length(), Line(b, c).length(), Line(c, a).length());
        assert!(close(ab, bc));

        let triangle_type = if ab < ca {
            assert!(close(ca / ab, PHI));
            RobinsonTriangleType::Large
        } else {
            assert!(close(ca / ab, PHI_INVERSE));
            RobinsonTriangleType::Small
        };
        RobinsonTriangle { triangle_type, a, b, c }
    }

    pub fn from_base(a: Point, c: Point, triangle_type: RobinsonTriangleType) -> Self {
        let ratio = match triangle_type {
            RobinsonTriangleType::Small => PHI,
            RobinsonTriangleType::Large => PHI_INVERSE,
        };
        let median = (a + c) / 2.0;
        // Normalized direction vector from the median to b.
        let direction_to_b = {
            let b_direction = c - a;
            Point(-b_direction.1, b_direction.0).normalized()
        };
        // Height of the result triangle.
        let height = {
            let base_length = Line(a, c).length();
            let hypotenuse = base_length * ratio;
            (hypotenuse.powi(2) - (base_length / 2.0).powi(2)).sqrt()
        };
        let b = median + height * direction_to_b;
        assert!(close(Line(a, b).length(), Line(b, c).length()));
        assert!(close(Line(a, b).length() / Line(a, c).length(), ratio));
        RobinsonTriangle { triangle_type, a, b, c }
    }

    pub fn lines(&self) -> [Line; 3] {
        [Line(self.a, self.b), Line(self.b, self.c), Line(self.c, self.a)]
    }
}
