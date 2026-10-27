use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Cylinder {
    center: Point3,
    radius: f64,
    height: f64,
}

impl Cylinder {
    pub fn new(center: Point3, radius: f64, height: f64) -> Cylinder {
        Cylinder {
            center,
            radius,
            height,
        }
    }
}

impl Hittable for Cylinder {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let bottom = self.center.y() - self.height / 2.0;
        let top = self.center.y() + self.height / 2.0;
        let origin_to_center = ray.origin() - self.center;
        let direction = ray.direction();

        let mut closest = t_max;
        let mut outward_normal = Vec3::default();
        let mut hit_anything = false;

        let a = direction.x() * direction.x() + direction.z() * direction.z();
        if a > 1e-8 {
            let half_b =
                origin_to_center.x() * direction.x() + origin_to_center.z() * direction.z();
            let c = origin_to_center.x() * origin_to_center.x()
                + origin_to_center.z() * origin_to_center.z()
                - self.radius * self.radius;
            let discriminant = half_b * half_b - a * c;

            if discriminant >= 0.0 {
                let square_root = discriminant.sqrt();

                for root in [(-half_b - square_root) / a, (-half_b + square_root) / a] {
                    if root <= t_min || closest <= root {
                        continue;
                    }

                    let point = ray.at(root);
                    if bottom < point.y() && point.y() < top {
                        closest = root;
                        outward_normal = Vec3::new(
                            (point.x() - self.center.x()) / self.radius,
                            0.0,
                            (point.z() - self.center.z()) / self.radius,
                        );
                        hit_anything = true;
                    }
                }
            }
        }

        if direction.y().abs() > 1e-8 {
            for (cap_y, cap_normal) in [
                (bottom, Vec3::new(0.0, -1.0, 0.0)),
                (top, Vec3::new(0.0, 1.0, 0.0)),
            ] {
                let root = (cap_y - ray.origin().y()) / direction.y();
                if root <= t_min || closest <= root {
                    continue;
                }

                let point = ray.at(root);
                let x = point.x() - self.center.x();
                let z = point.z() - self.center.z();
                if x * x + z * z <= self.radius * self.radius {
                    closest = root;
                    outward_normal = cap_normal;
                    hit_anything = true;
                }
            }
        }

        if !hit_anything {
            return false;
        }

        rec.t = closest;
        rec.p = ray.at(rec.t);
        rec.set_face_normal(ray, outward_normal);
        true
    }
}
