use std::fmt::Write as _;

use crate::{Array3, Double};
use crate::vec3::Vec3;
use crate::{gen_getter, vec3_op_scalar_and_op_assign, vec3_op_vec3_and_op_assign};

pub type RGB = Vec3<Color>;
#[derive(PartialEq, Debug)]
pub struct Color;
impl RGB {
    pub fn write_color(&self, buf: &mut String) {
        fn translate(old: Array3) -> [u8; 3] {
            let scalar = 255.999;
            old.map(|n| (scalar * n) as u8)
        }
        let [r, g, b] = translate(self.0);
        let _ = writeln!(buf, "{} {} {}", r, g, b);
    }
}
gen_getter! {RGB[r,g,b]=>Double}

vec3_op_vec3_and_op_assign! {
    [Add,Sub,Mul,Div]
    [add,sub,mul,div]
    [AddAssign,SubAssign,MulAssign,DivAssign]
    [add_assign,sub_assign,mul_assign,div_assign]
    RGB,RGB =>RGB
}
vec3_op_scalar_and_op_assign! {
    [Add,Sub,Mul,Div]
    [add,sub,mul,div]
    [AddAssign,SubAssign,MulAssign,DivAssign]
    [add_assign,sub_assign,mul_assign,div_assign]
    RGB,Double =>RGB
}
