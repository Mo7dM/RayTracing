use crate::ray::Ray;
use crate::vec3::{self, Point3};

pub fn hit_sphere(center:Point3,radius:f64,r:&Ray) -> bool{
    let oc = r.origin() - center;
    let a = vec3::dot(r.direction(),r.direction());
    let b = 2.0*vec3::dot(oc,r.direction());
    let c = vec3::dot(oc,oc)-radius*radius;
    let discriminant = b*b-4.0*a*c;
    discriminant>=0.0

}