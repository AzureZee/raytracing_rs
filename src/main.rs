use std::fmt::Write as _;
use std::fs;

use std::io::Write as _;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

fn main() {
    let img_w = 256;
    let img_h = 256;

    let mut canvas = String::new();
    let _ = write!(canvas, "P3\n{} {}\n255\n", img_w, img_h);

    for j in 0..img_h {
        eprint!("\rprogress: {}/{} ", j + 1, img_h);
        let _ = std::io::stderr().flush();

        for i in 0..img_w {
            let pixel = Pixel {
                e: [
                    double(i) / double(img_w - 1),
                    double(j) / double(img_h - 1),
                    0.0,
                ],
            };
            put_pixel(&mut canvas, pixel);
        }
    }
    let mut file = fs::File::create("img.ppm").unwrap();
    let _ = file.write_all(canvas.as_bytes());
}

fn double(i: i32) -> Double {
    i as Double
}

pub fn put_pixel(canvas: &mut String, pixel: Pixel) {
    fn translate(byte: Double) -> UInt {
        let n = 255.999;
        (n * byte) as UInt
    }
    let [r, g, b] = [
        translate(pixel.x()),
        translate(pixel.y()),
        translate(pixel.z()),
    ];
    let _ = writeln!(canvas, "{} {} {}", r, g, b);
}

type Point = Vec3;
type Pixel = Vec3;
type UInt = u64;
type Double = f64;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub e: [Double; 3],
}

impl Vec3 {
    pub fn x(&self) -> Double {
        self.e[0]
    }
    pub fn y(&self) -> Double {
        self.e[1]
    }
    pub fn z(&self) -> Double {
        self.e[2]
    }
    pub fn dot(&self, vec_b: &Vec3) -> Double {
        (*self * vec_b).e.iter().sum()
    }
    pub fn cross(&self, vec_b: &Vec3) -> Self {
        let vec_a = self;
        let c_z = vec_a.x() * vec_b.y() - vec_a.y() * vec_b.x();
        let c_y = vec_a.z() * vec_b.x() - vec_a.x() * vec_b.z();
        let c_x = vec_a.y() * vec_b.z() - vec_a.z() * vec_b.y();
        Self { e: [c_x, c_y, c_z] }
    }
    pub fn unit_vector(&self) -> Self {
        *self / self.len()
    }
    pub fn len(&self) -> Double {
        self.len_squared().sqrt()
    }
    pub fn len_squared(&self) -> Double {
        (*self * self).e.iter().sum()
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            e: [self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z()],
        }
    }
}
impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            e: [self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z()],
        }
    }
}
impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            e: [self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z()],
        }
    }
}
impl Mul<&Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        Self {
            e: [self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z()],
        }
    }
}
impl Mul<Double> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Double) -> Self::Output {
        Self {
            e: [self.x() * rhs, self.y() * rhs, self.z() * rhs],
        }
    }
}

impl MulAssign<Double> for Vec3 {
    fn mul_assign(&mut self, rhs: Double) {
        *self = *self * rhs;
    }
}
impl Div<Double> for Vec3 {
    type Output = Self;

    fn div(self, rhs: Double) -> Self::Output {
        self * (1.0 / rhs)
    }
}
impl DivAssign<Double> for Vec3 {
    fn div_assign(&mut self, rhs: Double) {
        *self = *self / rhs;
    }
}
impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            e: [-self.x(), -self.y(), -self.z()],
        }
    }
}
