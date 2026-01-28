use std::fmt::Write;
use std::io::Write as IoWrite;

fn main() {
    let img_w = 256;
    let img_h = 256;
    
    let mut ppm = String::new();
    let _ = write!(ppm, "P3\n{} {}\n255\n", img_w, img_h);

    for y in 0..img_h {
        
        eprint!("\rprogress: {}/{} ", y + 1, img_h);
        let _ = std::io::stderr().flush();

        for x in 0..img_w {
            let x = x as f64;
            let y = y as f64;
            let n = 255.999;
            let x_max = (img_w - 1) as f64;
            let y_max = (img_h - 1) as f64;

            let (r, g, b) = (x / x_max, y / y_max, 0.0);
            let (ir, ig, ib) = ((n * r) as u64, (n * g) as u64, (n * b) as u64);

            let _ = write!(ppm, "{} {} {}\n", ir, ig, ib);
        }
    }
    println!("{}", ppm.trim_end());
}
