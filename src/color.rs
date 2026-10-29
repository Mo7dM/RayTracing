use std::io::Write;

use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color(out: &mut impl Write,pixel_color : Color){
    let r = (255.999*pixel_color.x().clamp(0.0, 1.0)) as i32;
    let g = (255.999*pixel_color.y().clamp(0.0, 1.0)) as i32;
    let b = (255.999*pixel_color.z().clamp(0.0, 1.0)) as i32;
    writeln!(out, "{} {} {}", r, g, b).expect("writing color");
}
