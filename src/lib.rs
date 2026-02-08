#![feature(macro_metavar_expr)]
mod macros;
//
pub mod color;
pub mod hittable;
pub mod camera;
pub mod interval;
pub mod ray;
pub mod vec3;
pub type Array3 = [f64; 3];
pub type Double = f64;

pub trait AsDouble {
    fn as_double(self) -> Double;
    
}
macro_rules! as_double_impl {
    ($($type:ty),+) => {
        $(
        impl AsDouble for $type {
            fn as_double(self) -> Double {
                self as Double
            }
        })+
    };
}
as_double_impl!{u8,u32,u64,i32,i64}
