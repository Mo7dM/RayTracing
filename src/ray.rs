use std::f64::INFINITY;

use crate::{color::Color, hittable::{HitRecord, Hittable}, light::Light, vec3::{self, Point3, Vec3}};

#[derive(Default)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray{
    pub fn new(origin:Point3,direction:Vec3) -> Ray{
        Ray{
            orig:origin,dir:direction
        }
    }

    pub fn origin(&self) -> Point3{
        self.orig
    }

    pub fn direction(&self) -> Vec3{
        self.dir
    }

    pub fn at(&self, t:f64) -> Point3{
        self.orig + t*self.dir // P(t)=A+tb
    }
}

pub fn ray_color(r: &Ray, world: &dyn Hittable,light:&Light) -> Color {
    let mut rec = HitRecord::new();
    if world.hit(r, 0.0, INFINITY, &mut rec) {
        let direction_to_light = vec3::unit_vector(light.position - rec.p);
        let brightness = vec3::dot(rec.normal, direction_to_light).max(0.0) * light.intensity;
        return brightness * Color::new(1.0, 1.0, 1.0);
    }
    let unit_direction = vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}