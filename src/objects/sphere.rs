use crate::ray::Ray;
use crate::vec3::{self, Point3};
use crate::hittable::{HitRecord,Hittable};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(cen: Point3, r: f64) -> Sphere {
        Sphere {
            center: cen,
            radius: r,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = vec3::dot(oc, r.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrt_d = f64::sqrt(discriminant);

        // Find the nearest root that lies in the acceptable range
        let mut root = (-half_b - sqrt_d) / a;
        if root <= t_min || t_max <= root {
            root = (-half_b + sqrt_d) / a;
            if root <= t_min || t_max <= root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward);
        true
    }
}

// pub fn hit_sphere(center:Point3,radius:f64,r:&Ray) -> f64{
//     let oc = r.origin() - center;
//     let a = r.direction().length_squared();
//     let bhalf = vec3::dot(oc,r.direction());
//     let c = oc.length_squared()-radius*radius;
//     let discriminant = bhalf*bhalf-4.0*a*c;
//     // discriminant>=0.0
//     if discriminant <0.0{
//         -1.0
//     }else {
//         (-bhalf-f64::sqrt(discriminant))/a
//     }
// }