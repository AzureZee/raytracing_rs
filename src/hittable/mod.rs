use crate::{
    Double,
    interval::Interval,
    ray::Ray,
    vec3::{Point3, Vector3},
};
pub mod sphere;
pub type HittableBox = Box<dyn Hittable + 'static>;
pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t_range: Interval) -> Option<HitRecord>;
}

impl<H: Hittable + 'static> From<H> for HittableBox {
    fn from(value: H) -> Self {
        Box::new(value) as HittableBox
    }
}
impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t_range: Interval) -> Option<HitRecord> {
        let mut record = None;
        let mut range = ray_t_range;
        for obj in &self.objects {
            if let Some(temp_rec) = obj.hit(ray, range) {
                range.max = temp_rec.ray_t;
                record = Some(temp_rec)
            }
        }
        record
    }
}

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<HittableBox>,
}

impl From<Vec<HittableBox>> for HittableList {
    fn from(objects: Vec<HittableBox>) -> Self {
        Self { objects }
    }
}

impl HittableList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push<H: Hittable + 'static>(&mut self, obj: H) {
        self.objects.push(obj.into());
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

#[derive(Default)]
pub struct HitRecord {
    pub point: Point3,
    pub ray_t: Double,
    pub normal: Vector3,
    pub normal_direction: NormalDirection,
}

impl HitRecord {
    /// NOTE: the parameter `outward_normal` is assumed to have unit length.
    pub fn new(ray: &Ray, ray_t: Double, point: Point3, outward_normal: Vector3) -> Self {
        let (normal, normal_direction) = if ray.direction.dot(outward_normal) < 0.0 {
            //ray is outside the sphere
            (outward_normal, NormalDirection::Outward)
        } else {
            // ray is inside the sphere
            (-outward_normal, NormalDirection::Inward)
        };
        Self {
            point,
            normal,
            ray_t,
            normal_direction,
        }
    }
}
#[derive(Default)]
pub enum NormalDirection {
    #[default]
    Outward,
    Inward,
}

impl NormalDirection {
    /// Returns `true` if the normal direction is [`Outward`].
    ///
    /// [`Outward`]: NormalDirection::Outward
    #[must_use]
    pub fn is_outward(&self) -> bool {
        matches!(self, Self::Outward)
    }
}
