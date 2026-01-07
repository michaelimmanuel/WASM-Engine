use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg };

#[derive(Copy, Clone, Debug, PartialEq)]

pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    
    // CONSTANT
    pub const ZERO: Vec2 = Vec2 { x: 0.0, y: 0.0 };
    pub const ONE: Vec2 = Vec2 { x: 1.0, y: 1.0 };
    pub const X: Vec2 = Vec2 { x: 1.0, y: 0.0 };
    pub const Y: Vec2 = Vec2 { x: 0.0, y: 1.0 };

    #[inline]
    pub fn new( x:f32, y:f32 ) -> Self {
        Self { x, y}
    }

    #[inline]
    pub fn dot(self, other: Vec2) -> f32 {
        (self.x * other.x) + (self.y * other.y)
    }

    #[inline]
    pub fn cross(self, other: Vec2) -> f32 {
        (self.x * other.y) - (self.y * other.x)
    }

    #[inline]
    pub fn length_squared(self) -> f32 {
        self.dot(self)
    }

    #[inline]
    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    #[inline]
    pub fn normalize(self) -> Vec2 {
        let len = self.length();
        if len > 0.0 {
            self / len
        } else {
            Vec2::ZERO
        }
    }

    #[inline]
    pub fn distance(self, other: Vec2) -> f32 {
        (self - other).length()
    }

    #[inline]
    pub fn distance_squared(self, other: Vec2) -> f32 {
        (self - other).length_squared()
    }

    #[inline]
    pub fn is_zero(self) -> bool {
        self.x == 0.0 && self.y == 0.0
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    #[inline]
    fn add(self, right: Vec2) -> Vec2 {
        Vec2::new(self.x + right.x, self.y + right.y)
    }
}

impl AddAssign for Vec2 {
    #[inline]
    fn add_assign(&mut self, right: Vec2){
        self.x += right.x;
        self.y += right.y;
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    #[inline]
    fn sub(self, right: Vec2) -> Vec2 {
        Vec2::new(self.x - right.x, self.y - right.y)
    }
}

impl SubAssign for Vec2 {
    #[inline]
    fn sub_assign(&mut self, right: Vec2){
        self.x -= right.x;
        self.y -= right.y;
    }
}


impl Mul<f32> for Vec2 {
    type Output = Vec2;

    #[inline]
    fn mul(self, right: f32) -> Vec2 {
        Vec2::new(self.x * right, self.y * right)
    }
}

impl MulAssign<f32> for Vec2 {
    #[inline]
    fn mul_assign(&mut self, right: f32){
        self.x *= right;
        self.y *= right;
    }
}

impl Div<f32> for Vec2 {
    type Output = Vec2;

    #[inline]
    fn div(self, right: f32) -> Vec2 {
        Vec2::new(self.x / right, self.y / right)
    }
}

impl DivAssign<f32> for Vec2 {
    #[inline]
    fn div_assign(&mut self, right: f32){
        self.x /= right;
        self.y /= right;
    }
}

impl Neg for Vec2 {
    type Output = Vec2;

    #[inline]
    fn neg(self) -> Vec2 {
        Vec2::new(-self.x, -self.y)
    }
}

#[cfg(test)]
#[path = "tests/vec2_tests.rs"]
mod vec2_tests;



