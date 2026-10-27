use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::{self, Point3, Vec3};

pub struct Plane {
    point: Point3,
    normal: Vec3,
}

impl Plane {
    pub fn new(point: Point3, normal: Vec3) -> Plane {
        Plane {
            point,
            normal: vec3::unit_vector(normal),
        }
    }
}

impl Hittable for Plane {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let denominator = vec3::dot(self.normal, ray.direction());

        if denominator.abs() < 1e-8 {
            return false;
        }

        let root = vec3::dot(self.point - ray.origin(), self.normal) / denominator;
        if root <= t_min || t_max <= root {
            return false;
        }

        rec.t = root;
        rec.p = ray.at(rec.t);
        rec.set_face_normal(ray, self.normal);
        true
    }
}
