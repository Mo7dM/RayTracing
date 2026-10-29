mod color;
mod hittable;
mod light;
mod objects;
mod ray;
mod vec3;

use std::io;

use color::Color;
use hittable::{ColoredObject, HittableList};
use objects::cube::Cube;
use objects::cylinder::Cylinder;
use objects::plane::Plane;
use objects::sphere::Sphere;
use ray::Ray;
use ray::ray_color;
use vec3::{Point3, Vec3};

use crate::light::Light;

fn main() {
    // img config
    let args: Vec<String> = std::env::args().collect();
    let scene = args.get(1).map(String::as_str).unwrap_or("all");
    if !["sphere", "cube", "all", "alternate"].contains(&scene) || args.len() > 4 {
        eprintln!("Usage: cargo run -- [sphere|cube|all|alternate] [width height]");
        std::process::exit(1);
    }
    let dimensions = if args.len() == 2 || args.len() == 1 {
        Some((800, 600))
    } else if args.len() == 4 {
        args[2].parse::<i32>().ok().zip(args[3].parse::<i32>().ok())
    } else {
        None
    };
    let (img_width, img_height) = match dimensions {
        Some((width, height)) if width >= 2 && height >= 2 => (width, height),
        _ => { eprintln!("Width and height must both be integers of at least 2"); std::process::exit(1); }
    };
    let aspect_raito = img_width as f64/img_height as f64;
    // camera
    let viewport_height = 2.0*(60.0_f64.to_radians()/2.0).tan();
    let viewport_width = aspect_raito * viewport_height;
    let focal_length = 1.0;

    let camera_position = if scene == "alternate" {
        Point3::new(5.0, 3.0, 3.0) 
    } else {
        Point3::new(0.0, 1.0, 4.0) 
    };
    let target = Point3::new(0.0, -0.3, -4.0);
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
    if scene != "cube" {
        world.add(Box::new(ColoredObject::new(Box::new(Sphere::new(Point3::new(0.0, 0.0, -4.0), 1.0)), Color::new(1.0, 0.05, 0.05))));
    }
    if scene != "sphere" {
        let cube_x = if scene == "cube" { 0.0 } else { 2.0 };
        world.add(Box::new(ColoredObject::new(Box::new(Cube::new(Point3::new(cube_x, -0.5, -4.0), 1.0)), Color::new(0.05, 0.2, 1.0))));
    }
    if scene == "all" || scene == "alternate" {
        world.add(Box::new(ColoredObject::new(Box::new(Cylinder::new(Point3::new(-2.0, -0.25, -4.0), 0.5, 1.5)), Color::new(0.05, 0.8, 0.2))));
    }
    world.add(Box::new(ColoredObject::new(Box::new(Plane::new(
        Point3::new(0.0, -1.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    )), Color::new(0.6, 0.6, 0.6))));
    let light = Light::new(Vec3::new(-3.0, 5.0, 1.0), if scene == "cube" { 0.4 } else { 1.0 });
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
