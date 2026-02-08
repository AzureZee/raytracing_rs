// https://gabrielgambetta.com/computer-graphics-from-scratch/A0-linear-algebra.html
use crate::Double;
use crate::{Array3, gen_builder_lite, gen_getter, vec3_op_scalar, vec3_op_vec3};

use std::marker::PhantomData;
pub type Scalar = f64;
pub type Point3 = Vec3<_Point>;
pub type Vector3 = Vec3<_Vector>;

#[derive(PartialEq, Debug, Default)]
pub struct _Point;
#[derive(PartialEq, Debug, Default)]
pub struct _Vector;

#[derive(Debug, PartialEq, Default)]
pub struct Vec3<T>(pub Array3, PhantomData<T>);

impl<T> From<Array3> for Vec3<T> {
    fn from(value: Array3) -> Self {
        Self(value, PhantomData)
    }
}

impl<T> Vec3<T> {
    pub fn new(arr: Array3) -> Self {
        Self(arr, PhantomData)
    }
}
gen_getter! {
    Point3[x,y,z]=>f64
}
gen_getter! {
    Vector3[x,y,z]=>f64
}
gen_builder_lite! {
    Vector3
    [
        with_x,
        with_y,
        with_z
    ]
}
gen_builder_lite! {
    Point3
    [
        with_x,
        with_y,
        with_z
    ]
}
impl Vector3 {
    pub fn dot(&self, other: Vector3) -> Double {
        // dot product: multiply each of the coordinates and add each of the coordinates product
        // V dot W = V_x * W_x + V_y * W_y + V_z * W_z

        // This suggests another way to compute the length of a vector, as the square root of its dot product with itself
        // V dot V = V_x^2 + V_y^2 + V_z^2 = V_len^2
        (*self * other).0.iter().sum()
    }
    pub fn cross(&self, vec_b: Vector3) -> Self {
        // The cross product between two vectors gives you another vector,it perpendicular to both of them
        // R=V cross W
        // R=V_yzx * W_zxy - V_zxy * W_yzx
        // R_x=V_y * W_z − V_z * W_y
        // R_y=V_z * W_x − V_x * W_z
        // R_z=V_x * W_y − V_y * W_x
        let [a_x, a_y, a_z] = self.0;
        let [b_x, b_y, b_z] = vec_b.0;
        let c_x = a_y * b_z - a_z * b_y;
        let c_y = a_z * b_x - a_x * b_z;
        let c_z = a_x * b_y - a_y * b_x;
        Self::new([c_x, c_y, c_z])
    }

    pub fn unit_vector(&self) -> Self {
        // normalize a vector: into a unit vector, divide the vector by its length

        *self / self.len()
    }
    pub fn len(&self) -> Double {
        // vector magnitude (also called the length or norm).
        // vec(x,y,z), vec_len= sqrt(x^2 + y^2 + z^2)
        self.len_squared().sqrt()
    }
    pub fn len_squared(&self) -> Double {
        self.dot(*self)
    }
}

// Point:P,Q;Vector:V
// P-Q=V
vec3_op_vec3! {
    [Sub][sub]
    Point3,Point3=>Vector3
}
// Q+V=P
vec3_op_vec3! {
    [Add][add]
    Point3,Vector3=>Point3
}
// V+W,V-W,V*W,
vec3_op_vec3! {
    [Add,Sub,Mul,Div]
    [add,sub,mul,div]
    Vector3,Vector3=>Vector3
}
// V*scalar=V,
// V/scalar=V
vec3_op_scalar! {
    [Add,Sub,Mul,Div]
    [add,sub,mul,div]
    Vector3,Scalar=>Vector3
}

impl<T> std::ops::Deref for Vec3<T> {
    type Target = Array3;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> std::ops::DerefMut for Vec3<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Copy for Vec3<T> {}
impl<T> Clone for Vec3<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl std::ops::Neg for Vector3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new([-self.x(), -self.y(), -self.z()])
    }
}
