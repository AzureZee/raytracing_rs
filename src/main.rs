use raytracing_rs::color::*;
use raytracing_rs::vec3::*;
use std::fmt::Write as _;
use std::fs;
use std::io::Write as _;

fn main() {
    // image
    // w/h=16/9
    let img_w = 400;
    let aspect_ratio = 16.0 / 9.0;
    let img_h = int(double(img_w) / aspect_ratio);
    let img_h = if img_h < 1 { 1 } else { img_h };

    // camera
    // use right-handed coordinate system
    // y-axis go up, the x-axis to the right,
    // and the negative z-axis pointing in the viewing direction
    let focal_len = 1.0;
    let camera_center = Point3::new([0.0; 3]);
    let camera_direction = Vector3::new([0.0, 0.0, -focal_len]);

    let viewport_h = 2.0;
    let viewport_w = viewport_h * (double(img_w) / double(img_h));

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_horizontal = Vector3::new([viewport_w, 0.0, 0.0]);
    let viewport_vertical = Vector3::new([0.0, -viewport_h, 0.0]);

    let viewport_center = camera_center + camera_direction;
    let viewport_upper_left =
        viewport_center + ((-viewport_horizontal) + (-viewport_vertical)) * 0.5;

    let pixel_delta_horizontal = viewport_horizontal / double(img_w);
    let pixel_delta_vertical = viewport_vertical / double(img_h);

    let upper_left_pixel =
        viewport_upper_left + (pixel_delta_horizontal + pixel_delta_vertical) * 0.5;

    let mut buf = String::new();
    let _ = write!(buf, "P3\n{} {}\n255\n", img_w, img_h);

    for j in 0..img_h {
        eprint!("\rprogress: {}/{} ", j + 1, img_h);
        let _ = std::io::stderr().flush();

        for i in 0..img_w {
            let pixel_center = upper_left_pixel
                + pixel_delta_horizontal * double(i)
                + pixel_delta_vertical * double(j);

            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);
            ray.color().write_color(&mut buf);
        }
    }
    let mut file = fs::File::create("img.ppm").unwrap();
    let _ = file.write_all(buf.as_bytes());
}

fn double(i: i64) -> Double {
    i as Double
}
fn int(i: f64) -> Int {
    i as Int
}

type Double = f64;
type Int = i64;

pub struct Ray {
    origin: Point3,
    direction: Vector3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vector3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: Double) -> Point3 {
        self.origin + self.direction * t
    }
    pub fn color(&self) -> RGB {
        let sphere_center = Point3::new([0.0, 0.0, -1.0]);
        let sphere_radius = 0.5;
        let t = self.hit_sphere(sphere_center, sphere_radius);
        if t > 0.0 {
            // sphere color
            //visualizing normals 可视化法向量
            // normals.xyz() ∈ [-1, 1], + 1 → ∈ [0, 2],* 0.5 → ∈ [0, 1]
            // map normals.xyz => rgb
            let normals = (self.at(t) - sphere_center).unit_vector();
            let rgb = normals.map(|n| (n + 1.0) * 0.5);
            return RGB::new(rgb);
        }

        // unit_vector.y() ∈ [-1, 1], + 1 → ∈ [0, 2],* 0.5 → ∈ [0, 1]
        let unit_vector = self.direction.unit_vector();
        //interpolation factor
        let factor = (unit_vector.y() + 1.0) * 0.5;
        // background color
        let white = RGB::new([1.0; 3]);
        let blue = RGB::new([0.5, 0.7, 1.0]);
        // linear blend / lerp
        white * (1.0 - factor) + blue * factor
    }
    /// 球心 C, 球面上的点 P, 半径r
    /// (C-P)dot(C-P)=r^2
    /// 任意满足此方程的点 P 在球面上
    /// O=ray.origin, d=ray.direction, 实数 t
    /// ray上的点 P(t)=O+t*d
    /// 如果ray与sphere相交, 则
    /// (C-(O+t*d))dot(C-(O+t*d))=r^2
    /// 因为 -(O+t*d)=-O-td
    /// let OC=C-O
    /// (-td+OC)dot(-td+OC)=r^2
    ///
    /// 点积符合乘法分配律,以及交换律 V dot W = W dot V;标量积可分配,即 k * (V dot W) = k*V dot W
    /// t^2 * d dot d + -td dot OC + -td dot OC + OC dot OC
    /// t^2 * d dot d + t*(-2*(d dot OC)) + OC dot OC - r^2=0
    ///
    /// 二次方程 a*t^2 + b*t + c =0
    /// a = d dot d, b=-2d dot OC, c=OC dot OC - r^2
    /// 求根公式 t =( -b +- sqrt(b^2 - 4ac) )/2a ,
    /// 判别式 q=b^2 - 4ac, q< 0, 无解; q=0, 一个解; q>0, 有两个解
    /// 分别对应 射线不与球面相交、射线与球面相切以及射线进入和离开球面
    /// 在两个解的情况下 有t1, t2
    /// 如果 射线进入 ,t1=( -b - sqrt(b^2 - 4ac) )/2a
    /// 如果 射线离开 ,t2=( -b + sqrt(b^2 - 4ac) )/2a
    /// 如果 相切 t1=t2
    /// 取最近的t, 即t1
    fn hit_sphere(&self, center: Point3, radius: Double) -> Double {
        // the vector from ray_origin to sphere_center OC
        let oc = center - self.origin;
        //
        let a = self.direction.len_squared();
        // if b = -2h = -2d dot OC
        // h = d dot OC
        // t =( -b +- sqrt(b^2 - 4ac) )/2a = (h +- sqrt(h^2-ac))/a
        let h = self.direction.dot(oc);
        let c = oc.len_squared() - radius * radius;
        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return -1.0;
        } else {
            return (h - discriminant.sqrt()) / a;
        }
    }
}
