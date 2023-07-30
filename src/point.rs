#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub const ZERO: Self = Self { x: 0., y: 0. };
    pub const ONE: Self = Self { x: 1., y: 1. };
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    pub fn round(&mut self) {
        *self = self.rounded()
    }
    pub fn rounded(&self) -> Self {
        Self::new(self.x.round(), self.y.round())
    }

    pub fn floor(&mut self) {
        *self = self.floored()
    }
    pub fn floored(&self) -> Self {
        Self::new(self.x.floor(), self.y.floor())
    }
}

impl std::convert::From<Point> for ggez::mint::Point2<f32> {
    fn from(o: Point) -> Self {
        ggez::mint::Point2 {
            x: o.x as f32,
            y: o.y as f32,
        }
    }
}
impl std::convert::From<Point> for ggez::mint::Point2<f64> {
    fn from(o: Point) -> Self {
        ggez::mint::Point2 { x: o.x, y: o.y }
    }
}
impl std::convert::From<Point> for ggez::mint::Vector2<f32> {
    fn from(o: Point) -> Self {
        ggez::mint::Vector2 {
            x: o.x as f32,
            y: o.y as f32,
        }
    }
}
impl std::convert::From<Point> for ggez::mint::Vector2<f64> {
    fn from(o: Point) -> Self {
        ggez::mint::Vector2 { x: o.x, y: o.y }
    }
}
impl std::convert::From<ggez::mint::Point2<f32>> for Point {
    fn from(o: ggez::mint::Point2<f32>) -> Self {
        Point {
            x: o.x as f64,
            y: o.y as f64,
        }
    }
}
impl std::convert::From<ggez::mint::Point2<f64>> for Point {
    fn from(o: ggez::mint::Point2<f64>) -> Self {
        Point { x: o.x, y: o.y }
    }
}
impl std::convert::From<ggez::mint::Vector2<f32>> for Point {
    fn from(o: ggez::mint::Vector2<f32>) -> Self {
        Point {
            x: o.x as f64,
            y: o.y as f64,
        }
    }
}
impl std::convert::From<ggez::mint::Vector2<f64>> for Point {
    fn from(o: ggez::mint::Vector2<f64>) -> Self {
        Point { x: o.x, y: o.y }
    }
}

impl std::convert::From<(u32, u32)> for Point {
    fn from(other: (u32, u32)) -> Self {
        Point {
            x: other.0 as f64,
            y: other.1 as f64,
        }
    }
}
impl std::convert::From<(f32, f32)> for Point {
    fn from(other: (f32, f32)) -> Self {
        Point {
            x: other.0 as f64,
            y: other.1 as f64,
        }
    }
}
impl std::convert::From<(f64, f64)> for Point {
    fn from(other: (f64, f64)) -> Self {
        Point {
            x: other.0,
            y: other.1,
        }
    }
}
impl std::convert::From<Point> for (f64, f64) {
    fn from(other: Point) -> Self {
        (other.x, other.y)
    }
}

impl std::convert::From<(usize, usize)> for Point {
    fn from(other: (usize, usize)) -> Self {
        Point {
            x: other.0 as f64,
            y: other.1 as f64,
        }
    }
}
impl std::convert::From<Point> for (usize, usize) {
    fn from(other: Point) -> Self {
        (other.x as usize, other.y as usize)
    }
}

impl std::convert::From<f64> for Point {
    fn from(other: f64) -> Self {
        Point::new(other, other)
    }
}
impl std::ops::Add<Self> for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}
impl std::ops::Add<f64> for Point {
    type Output = Self;

    fn add(self, other: f64) -> Point {
        Self {
            x: self.x + other,
            y: self.y + other,
        }
    }
}
impl std::ops::Sub<Self> for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl std::ops::SubAssign for Point {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}
impl std::ops::Sub<f64> for Point {
    type Output = Self;

    fn sub(self, other: f64) -> Self {
        Self {
            x: self.x - other,
            y: self.y - other,
        }
    }
}
impl std::ops::Div<Point> for Point {
    type Output = Self;

    fn div(self, other: Point) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}
impl std::ops::Div<f64> for Point {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
        }
    }
}
impl std::ops::Mul<Point> for Point {
    type Output = Self;

    fn mul(self, other: Point) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}
impl std::ops::Mul<f64> for Point {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}
impl std::ops::Neg for Point {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
