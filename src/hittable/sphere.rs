use crate::{Array3, Double, hittable::Hittable, interval::Interval, ray::Ray, vec3::Point3};

use super::HitRecord;

pub struct Sphere {
    pub center: Point3,
    pub radius: Double,
}

impl Hittable for Sphere {
    // 球心 C, 球面上的点 P, 半径r
    // (C-P)dot(C-P)=r^2
    // 任意满足此方程的点 P 在球面上
    // O=ray.origin, d=ray.direction, 实数 t
    // ray上的点 P(t)=O+t*d
    // 如果ray与sphere相交, 则
    // (C-(O+t*d))dot(C-(O+t*d))=r^2
    // 因为 -(O+t*d)=-O-td
    // let OC=C-O
    // (-td+OC)dot(-td+OC)=r^2
    //
    // 点积符合乘法分配律,以及交换律 V dot W = W dot V;标量积可分配,即 k * (V dot W) = k*V dot W
    // t^2 * d dot d + -td dot OC + -td dot OC + OC dot OC
    // t^2 * d dot d + t*(-2*(d dot OC)) + OC dot OC - r^2=0
    //
    // 二次方程 a*t^2 + b*t + c =0
    // a = d dot d, b=-2d dot OC, c=OC dot OC - r^2
    // 求根公式 t =( -b +- sqrt(b^2 - 4ac) )/2a ,
    // 判别式 q=b^2 - 4ac, q< 0, 无解; q=0, 一个解; q>0, 有两个解
    // 分别对应 射线不与球面相交、射线与球面相切以及射线进入和离开球面
    // 在两个解的情况下 有t1, t2
    // 如果 射线进入 ,t1=( -b - sqrt(b^2 - 4ac) )/2a
    // 如果 射线离开 ,t2=( -b + sqrt(b^2 - 4ac) )/2a
    // 如果 相切 t1=t2
    // 取最近的t, 即t1
    fn hit(&self, ray: &Ray, ray_t_range: Interval) -> Option<HitRecord> {
        // the vector from ray_origin to sphere_center OC
        let oc = self.center - ray.origin;
        //
        let a = ray.direction.len_squared();
        // if b = -2h = -2d dot OC
        // h = d dot OC
        // t =( -b +- sqrt(b^2 - 4ac) )/2a = (h +- sqrt(h^2-ac))/a
        let h = ray.direction.dot(oc);
        let c = oc.len_squared() - self.radius.powf(2.0);
        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();
        let mut root = (h - sqrt_d) / a;
        if !ray_t_range.surrounds(root) {
            root = (h + sqrt_d) / a;
            if !ray_t_range.surrounds(root) {
                return None;
            }
        }

        let point = ray.at(root);
        // unit vector: vector(P-C)/len(radius)
        let outward_normal = (point - self.center) / self.radius;
        let record = HitRecord::new(ray, root, point, outward_normal);

        Some(record)
    }
}

impl Sphere {
    pub fn from_array(center: Array3) -> Self {
        Self {
            center: center.into(),
            radius: Default::default(),
        }
    }
    pub fn from_point3(center: Point3) -> Self {
        Self {
            center,
            radius: Default::default(),
        }
    }
    pub fn with_radius(mut self,radius: Double) ->Self {
        self.radius=radius;
        self
    }
    pub fn new(center: Array3, radius: Double) -> Self {
        Self {
            center: center.into(),
            radius: radius.max(0.0),
        }
    }
}
