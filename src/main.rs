use raytracing_rs::camera::Camera;
use raytracing_rs::hittable::HittableList;
use raytracing_rs::hittable::sphere::Sphere;
use raytracing_rs::vec3::Point3;
fn main() {
    //world
    let sphere_small = Sphere::from_point3(Point3::default().with_z(-1.0)).with_radius(0.5);
    let sphere_large = Sphere::from_array([0.0,-100.5,-1.0]).with_radius(100.0);
    let world = HittableList::from(vec![
        sphere_small.into(),
        sphere_large.into(),
    ]);
    let image_width = 400;
    let aspect_ratio = 16.0 / 9.0;
    let camera = Camera::new(aspect_ratio, image_width);
    camera.render(world);
}
