use std::fmt::Write as _;
use std::fs;
use std::io::Write as _;

mod vec3;
use vec3::*;
fn main() {
    let img_w = 256;
    let img_h = 256;

    let mut buf = String::new();
    let _ = write!(buf, "P3\n{} {}\n255\n", img_w, img_h);

    for j in 0..img_h {
        eprint!("\rprogress: {}/{} ", j + 1, img_h);
        let _ = std::io::stderr().flush();

        for i in 0..img_w {
            let color = RGB::new([
                double(i) / double(img_w - 1),
                double(j) / double(img_h - 1),
                0.0,
            ]);
            color.write_color(&mut buf);
        }
    }
    let mut file = fs::File::create("img.ppm").unwrap();
    let _ = file.write_all(buf.as_bytes());
}

fn double(i: i32) -> Double {
    i as Double
}

type Double = f64;

pub struct Ray {
    origin: Point3,
    direction: Vector3,
}

impl Ray {
    pub fn at(&self, t: Double) -> Point3 {
        self.origin + self.direction * t
    }
}
