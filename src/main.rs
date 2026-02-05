use raytracing_rs::Double;
use raytracing_rs::color::*;
use raytracing_rs::hittable::Hittable;
use raytracing_rs::hittable::HittableList;
use raytracing_rs::hittable::sphere::Sphere;
use raytracing_rs::interval::Interval;
use raytracing_rs::ray::Ray;
use raytracing_rs::vec3::*;
use std::fmt::Write as _;
use std::fs;
use std::io::Write as _;
use std::sync::mpsc;
use std::thread;
use std::time::Instant;

fn main() {
    // image
    // w/h=16/9
    let img_w = 400;
    let aspect_ratio = 16.0 / 9.0;
    let img_h = uint(double(img_w) / aspect_ratio);
    let img_h = if img_h < 1 { 1 } else { img_h };
    //world
    let sphere_down_right = Sphere::new([ 0.5, -0.5, -2.5], 0.5).into();
    let sphere_down_left  = Sphere::new([-0.5, -0.5, -2.5], 0.5).into();
    let sphere_up_right   = Sphere::new([ 0.5,  0.5, -2.5], 0.5).into();
    let sphere_up_left    = Sphere::new([-0.5,  0.5, -2.5], 0.5).into();
    let world = HittableList::from(vec![
        sphere_down_right,
        sphere_down_left,
        sphere_up_right,
        sphere_up_left,
    ]);

    // camera
    // use right-handed coordinate system
    // y-axis go up, the x-axis to the right,
    // and the negative z-axis pointing in the viewing direction
    let focal_len = 1.0;
    let camera_center = Point3::default();
    let camera_direction = Vector3::default().with_z(-focal_len);

    let viewport_h = 2.0;
    let ratio = double(img_w) / double(img_h);
    let viewport_w = viewport_h * ratio;

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_horizontal = Vector3::default().with_x(viewport_w);
    let viewport_vertical = Vector3::default().with_y(-viewport_h);

    let viewport_center = camera_center + camera_direction;
    let viewport_upper_left =
        viewport_center + ((-viewport_horizontal) + (-viewport_vertical)) * 0.5;

    let pixel_delta_horizontal = viewport_horizontal / double(img_w);
    let pixel_delta_vertical = viewport_vertical / double(img_h);

    let upper_left_pixel =
        viewport_upper_left + (pixel_delta_horizontal + pixel_delta_vertical) * 0.5;
    // println!(
    //     "aspect_ratio{{img:{aspect_ratio},vp:{ratio}}}\n\
    //     img{{w:{img_w},h:{img_h}}}\n\
    //     viewport{{w:{viewport_w},h:{viewport_h}}}\n\
    //     pixel{{w:{},h:{}}}",
    //     pixel_delta_horizontal.len(),
    //     pixel_delta_vertical.len()
    // );

    let mut buf = String::new();
    let _ = write!(buf, "P3\n{} {}\n255\n", img_w, img_h);

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        while let Ok(res) = rx.recv() {
            eprint!("\rprogress: {}/{} ", res, img_h);
            let _ = std::io::stderr().flush();
        }
    });
    let start = Instant::now();
    for j in 0..img_h {
        let _ = tx.send(j + 1);

        for i in 0..img_w {
            let pixel_center = upper_left_pixel
                + pixel_delta_horizontal * double(i)
                + pixel_delta_vertical * double(j);

            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);
            color(&ray, &world).write_color(&mut buf);
        }
    }
    let stop = start.elapsed();
    let mut file = fs::File::create("img.ppm").unwrap();
    let _ = file.write_all(buf.as_bytes());
    println!("stop:{stop:?}");
}

fn uint(i: f64) -> u64 {
    i as u64
}

fn double(i: u64) -> Double {
    i as Double
}

pub fn color<H: Hittable>(ray: &Ray, world: &H) -> RGB {
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
