mod vec3;
mod color;

use std::io;

use color::Color;

fn main() {
    let img_width = 256;
    let img_height = 256;
    println!("P3\n {img_height} {img_width}\n255");

    for j in 0..img_height {
        eprint!("\rScanning Remaining {}", j );
        for i in 0..img_width{
            let r = i as f64 / (img_width-1) as f64;
            let g = j as f64 / (img_height-1) as f64;
            let b= 0.25;
            let pixel_color = Color::new(r,g,b);
            color::write_color(&mut io::stdout(),pixel_color);
            let ir = (255.999*r) as usize;
            let ig = (255.999*g) as usize;
            let ib = (255.999*b) as usize;
            println!("{ir} {ig} {ib}")
        }
    }
    eprintln!("\rDone.\n")
}
