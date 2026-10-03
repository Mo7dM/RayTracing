use std::io::Write;


pub struct Color {
    r:f64,
    g:f64,
    b:f64
}

impl Color {
    pub fn new(r:f64,g:f64,b:f64)->Color{
        Color{r,g,b}
    }
}

pub fn write_color(out: &mut impl Write,pixel_color : Color){
    let r = (255.999*pixel_color.r) as usize;
    let g = (255.999*pixel_color.r) as usize;
    let b = (255.999*pixel_color.r) as usize;
}
