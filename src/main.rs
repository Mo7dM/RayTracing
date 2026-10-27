mod vec3;
mod color;
mod ray;
mod objects;
mod hittable;
mod light;

use std::io;

use color::Color;
use ray::Ray;
use vec3::{Point3,Vec3};
use objects::plane::Plane;
use objects::sphere::Sphere;
use hittable::{HitRecord,Hittable,HittableList};
use std::f64::INFINITY; // this thing i will be change to another file
use ray::ray_color;

use crate::light::Light;

fn main() {
    // img config
    let aspect_raito = 16.0/9.0;
    let img_width = 400;
    let img_height = (img_width as f64/aspect_raito) as i32;
    // camera
    let viewport_height = 2.0;
    let viewport_width = aspect_raito * viewport_height;
    let focal_length = 1.0;

    let camera_position = Point3::new(1.0, 0.5, 0.0);
    let target = Point3::new(0.0, 0.0, -1.0);
    let world_up = Vec3::new(0.0, 1.0, 0.0);

    let camera_back = vec3::unit_vector(camera_position - target);
    let camera_right = vec3::unit_vector(vec3::cross(world_up, camera_back));
    let camera_up = vec3::cross(camera_back, camera_right);

    let origin = camera_position;
    let horizontal = viewport_width * camera_right;
    let vertical = viewport_height * camera_up;
    let lowleftcorner = origin - horizontal / 2.0 - vertical / 2.0 - focal_length * camera_back;

    println!("P3\n {img_width} {img_height}\n255");
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Plane::new(
        Point3::new(0.0, -0.5, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    )));
    let light = Light::new(Vec3::new(-3.0, 5.0, 1.0), 1.0);
    for j in (0..img_height).rev() {
        eprint!("\rScanning Remaining {}", j );
        for i in 0..img_width{
            let u = i as f64 / (img_width-1) as f64;
            let v = j as f64 / (img_height-1) as f64;
            let r= Ray::new(origin, lowleftcorner+u*horizontal+v*vertical-origin);
            let pixel_color = ray_color(&r,&world,&light);
            color::write_color(&mut io::stdout(), pixel_color);
        }
    }
    eprintln!("\rDone.\n")
}
