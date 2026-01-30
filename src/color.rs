use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub};
use std::fmt::Write as _;

use crate::vec3::Vec3;
type Number = f64;

pub type RGB = Vec3<Color>;
#[derive(PartialEq, Debug)]
pub struct Color;
impl RGB {
    pub fn r(&self) -> Number {
        self.arr[0]
    }
    pub fn g(&self) -> Number {
        self.arr[1]
    }
    pub fn b(&self) -> Number {
        self.arr[2]
    }
    pub fn write_color(&self, buf: &mut String) {
        fn translate(old: [Number; 3]) -> [u64; 3] {
            let n = 255.999;
            let mut new = [0; 3];
            for i in 0..old.len() {
                new[i] = (n * old[i]) as u64;
            }
            new
        }
        let [r, g, b] = translate(self.arr);
        let _ = writeln!(buf, "{} {} {}", r, g, b);
    }
}
impl Add for RGB {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            [self.r() + rhs.r(), self.g() + rhs.g(), self.b() + rhs.b()]
        )
    }
}

impl AddAssign for RGB {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
impl Sub for RGB {
    type Output=Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(
            [self.r() - rhs.r(), self.g() - rhs.g(), self.b() - rhs.b()]
        )
    }
}
impl Mul<Number> for RGB {
    type Output = Self;

    fn mul(self, rhs: Number) -> Self::Output {
        Self::new(
            [self.r() * rhs, self.g() * rhs, self.b() * rhs]
        )
    }
}

impl MulAssign<Number> for RGB {
    fn mul_assign(&mut self, rhs: Number) {
        *self = *self * rhs;
    }
}
impl Div<Number> for RGB {
    type Output = Self;
    fn div(self, rhs: Number) -> Self::Output {
        self * (1.0 / rhs)
    }
}
impl DivAssign<Number> for RGB {
    fn div_assign(&mut self, rhs: Number) {
        *self = *self / rhs;
    }
}