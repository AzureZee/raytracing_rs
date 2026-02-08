use crate::{
    AsDouble, Double,
    color::RGB,
    hittable::Hittable,
    interval::Interval,
    ray::Ray,
    vec3::{Point3, Vector3},
};
use std::fmt::Write as _;
use std::fs;
use std::io::Write as _;

pub struct Camera {
    pub aspect_ratio: Double,
    pub image_width: u32,
    image_height: u32,
    origin: Point3,
    start_pixel: Point3,
    pixel_offset: Offset,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            image_height: Default::default(),
            origin: Default::default(),
            start_pixel: Default::default(),
            pixel_offset: Default::default(),
        }
    }
}

impl Camera {
    pub fn new(aspect_ratio: Double, image_width: u32) -> Self {
        struct Viewport {
            upper_left: Point3,
            horizontal: Vector3,
            vertical: Vector3,
        }
        // image
        // w/h=16/9
        let image_height = (image_width.as_double() / aspect_ratio) as u32;
        let image_height = if image_height < 1 { 1 } else { image_height };
        // camera
        // use right-handed coordinate system
        // y-axis go up, the x-axis to the right,
        // and the negative z-axis pointing in the viewing direction
        let focal_len = 1.0;
        let camera_origin = Point3::default();
        let camera_direction = Vector3::default().with_z(-focal_len);
        let viewport = {
            let height = 2.0;
            let ratio = image_width.as_double() / image_height.as_double();
            let width = height * ratio;
            let horizontal = Vector3::default().with_x(width);
            let vertical = Vector3::default().with_y(-height);
            let center = camera_origin + camera_direction;
            let upper_left = center + ((-horizontal) + (-vertical)) * 0.5;
            Viewport {
                upper_left,
                horizontal,
                vertical,
            }
        };
        let pixel_offset = Offset {
            horizontal: viewport.horizontal / image_width.as_double(),
            vertical: viewport.vertical / image_height.as_double(),
        };
        let start_pixel =
            viewport.upper_left + (pixel_offset.horizontal + pixel_offset.vertical) * 0.5;
        Self {
            aspect_ratio,
            image_width,
            image_height,
            origin: camera_origin,
            start_pixel,
            pixel_offset,
        }
    }
    pub fn render(self, world: impl Hittable) {
        let Camera {
            image_width,
            image_height,
            origin,
            start_pixel,
            pixel_offset,
            ..
        } = self;
        let mut buf = String::new();
        let _ = write!(buf, "P3\n{} {}\n255\n", image_width, image_height);

        for j in 0..image_height {
            for i in 0..image_width {
                let pixel_center = start_pixel
                    + pixel_offset.horizontal * i.as_double()
                    + pixel_offset.vertical * j.as_double();

                let ray_direction = pixel_center - origin;
                let ray = Ray::new(origin, ray_direction);
                ray_color(&ray, &world).write_color(&mut buf);
            }
        }
        match fs::File::create("img.ppm") {
            Ok(mut file) => {
                let _ = file.write_all(buf.as_bytes());
            }
            Err(err) => println!("{:?}", err),
        }
    }
}

#[derive(Default, Clone, Copy)]
struct Offset {
    horizontal: Vector3,
    vertical: Vector3,
}
fn ray_color(ray: &Ray, world: &impl Hittable) -> RGB {
    if let Some(record) = world.hit(ray, Interval::new(0.0, f64::INFINITY)) {
        // world color
        // visualizing normal 可视化法向量
        // normal.xyz() ∈ [-1, 1], + 1 → ∈ [0, 2],* 0.5 → ∈ [0, 1]
        // map normal.xyz => rgb
        let rgb = record.normal.map(|n| (n + 1.0) * 0.5);
        return RGB::new(rgb);
    }

    // unit_vector.y() ∈ [-1, 1], + 1 → ∈ [0, 2],* 0.5 → ∈ [0, 1]
    let unit_vector = ray.direction.unit_vector();
    //interpolation factor
    let factor = (unit_vector.y() + 1.0) * 0.5;
    // background color
    let white = RGB::new([1.0; 3]);
    let blue = RGB::new([0.5, 0.7, 1.0]);
    // linear blend / lerp
    white * (1.0 - factor) + blue * factor
}
