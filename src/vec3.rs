use std::{
    marker::PhantomData,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub},
};
type Number = f64;

pub type Point3 = Vec3<Point>;
pub type Vector3 = Vec3<Vector>;

#[derive(PartialEq, Debug)]
pub struct Point;
#[derive(PartialEq, Debug)]
pub struct Vector;

// https://gabrielgambetta.com/computer-graphics-from-scratch/A0-linear-algebra.html

#[derive(Debug, PartialEq)]
pub struct Vec3<T> {
    pub arr: [Number; 3],
    _marker: PhantomData<T>,
}

impl<T> Copy for Vec3<T> {}
impl<T> Clone for Vec3<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl HasCoordinates for Point {}
impl HasCoordinates for Vector {}
pub trait HasCoordinates {}
impl<T> Vec3<T> {
    pub fn new(arr: [Number; 3]) -> Self {
        Self {
            arr,
            _marker: PhantomData,
        }
    }
}
impl<T: HasCoordinates> Vec3<T> {
    pub fn x(&self) -> Number {
        self.arr[0]
    }
    pub fn y(&self) -> Number {
        self.arr[1]
    }
    pub fn z(&self) -> Number {
        self.arr[2]
    }
}

impl Vector3 {
    pub fn dot(&self, other: Vector3) -> Number {
        // dot product: multiply each of the coordinates and add each of the coordinates product
        // V dot W = V_x * W_x + V_y * W_y + V_z * W_z

        // This suggests another way to compute the length of a vector, as the square root of its dot product with itself
        // V dot V = V_x^2 + V_y^2 + V_z^2 = V_len^2
        (*self * other).arr.iter().sum()
    }

    pub fn cross(&self, other: Vector3) -> Self {
        // The cross product between two vectors gives you another vector,it perpendicular to both of them
        // R=V cross W
        // R_x=V_y * W_z − V_z * W_y
        // R_y=V_z * W_x − V_x * W_z
        // R_z=V_x * W_y − V_y * W_x
        let v_xyz = self.clone();
        let w_xyz = other;
        let mut v_zxy = v_xyz;
        let mut w_zxy = w_xyz;
        let mut v_yzx = v_zxy;
        let mut w_yzx = w_zxy;

        // [(x,y),z]=>[z,(x,y)]
        v_zxy.arr.rotate_left(2);
        w_zxy.arr.rotate_left(2);
        // [x,(y,z)]=>[(y,z),x]
        v_yzx.arr.rotate_right(2);
        w_yzx.arr.rotate_right(2);

        v_yzx * w_zxy - v_zxy * w_yzx
    }
    pub fn unit_vector(&self) -> Self {
        // normalize a vector: into a unit vector, divide the vector by its length

        *self / self.len()
    }
    pub fn len(&self) -> Number {
        // vector magnitude (also called the length or norm).
        // vec(x,y,z), vec_len= sqrt(x^2 + y^2 + z^2)
        self.len_squared().sqrt()
    }
    pub fn len_squared(&self) -> Number {
        self.dot(*self)
    }
    fn _cross(&self, vec_b: Vector3) -> Self {
        let vec_a = self;
        let c_z = vec_a.x() * vec_b.y() - vec_a.y() * vec_b.x();
        let c_y = vec_a.z() * vec_b.x() - vec_a.x() * vec_b.z();
        let c_x = vec_a.y() * vec_b.z() - vec_a.z() * vec_b.y();
        Self {
            arr: [c_x, c_y, c_z],
            _marker: PhantomData,
        }
    }
}

impl<T: HasCoordinates> Add<Vector3> for Vec3<T> {
    type Output = Self;
    // add a vector to a point and get a new point
    // or
    // add two vectors and get a new vector
    fn add(self, rhs: Vector3) -> Self::Output {
        Self {
            arr: [self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z()],
            _marker: PhantomData,
        }
    }
}

impl<T: HasCoordinates> AddAssign<Vector3> for Vec3<T> {
    fn add_assign(&mut self, rhs: Vector3) {
        *self = *self + rhs;
    }
}
impl<T: HasCoordinates> Sub for Vec3<T> {
    // subtract two points and get a vector, subtract each of the coordinates separately

    type Output = Vector3;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector3 {
            arr: [self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z()],
            _marker: PhantomData,
        }
    }
}
impl Mul for Vector3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            arr: [self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z()],
            _marker: PhantomData,
        }
    }
}

impl Mul<Number> for Vector3 {
    type Output = Self;
    // multiply a vector by a number k and get the scalar product.
    // This makes the vector shorter or longer

    fn mul(self, rhs: Number) -> Self::Output {
        Self {
            arr: [self.x() * rhs, self.y() * rhs, self.z() * rhs],
            _marker: PhantomData,
        }
    }
}

impl MulAssign<Number> for Vector3 {
    fn mul_assign(&mut self, rhs: Number) {
        *self = *self * rhs;
    }
}
impl Div<Number> for Vector3 {
    type Output = Self;
    // divide a vector by a number k.
    // dividing by k is equivalent to multiplying by 1/k

    fn div(self, rhs: Number) -> Self::Output {
        self * (1.0 / rhs)
    }
}
impl DivAssign<Number> for Vector3 {
    fn div_assign(&mut self, rhs: Number) {
        *self = *self / rhs;
    }
}
impl Neg for Vector3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            arr: [-self.x(), -self.y(), -self.z()],
            _marker: PhantomData,
        }
    }
}
