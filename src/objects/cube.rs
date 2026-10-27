use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Cube {
    min: Point3,
    max: Point3,
}

impl Cube {
    pub fn new(center: Point3, side_length: f64) -> Cube {
        let half_side = side_length / 2.0;
        Cube {
            min: center - Vec3::new(half_side, half_side, half_side),
            max: center + Vec3::new(half_side, half_side, half_side),
        }
    }
}

fn update_interval(
    origin: f64,
    direction: f64,
    min: f64,
    max: f64,
    min_normal: Vec3,
    max_normal: Vec3,
    enter: &mut f64,
    exit: &mut f64,
    enter_normal: &mut Vec3,
    exit_normal: &mut Vec3,
) -> bool {
    if direction.abs() < 1e-8 {
        return min <= origin && origin <= max;
    }

    let near = (min - origin) / direction;
    let far = (max - origin) / direction;
    let (near, far, near_normal, far_normal) = if near <= far {
        (near, far, min_normal, max_normal)
    } else {
        (far, near, max_normal, min_normal)
    };

    if near > *enter {
        *enter = near;
        *enter_normal = near_normal;
    }
    if far < *exit {
        *exit = far;
        *exit_normal = far_normal;
    }

    *enter <= *exit
}

impl Hittable for Cube {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut enter = f64::NEG_INFINITY;
        let mut exit = f64::INFINITY;
        let mut enter_normal = Vec3::default();
        let mut exit_normal = Vec3::default();

        if !update_interval(
            ray.origin().x(),
            ray.direction().x(),
            self.min.x(),
            self.max.x(),
            Vec3::new(-1.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            &mut enter,
            &mut exit,
            &mut enter_normal,
            &mut exit_normal,
        ) || !update_interval(
            ray.origin().y(),
            ray.direction().y(),
            self.min.y(),
            self.max.y(),
            Vec3::new(0.0, -1.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            &mut enter,
            &mut exit,
            &mut enter_normal,
            &mut exit_normal,
        ) || !update_interval(
            ray.origin().z(),
            ray.direction().z(),
            self.min.z(),
            self.max.z(),
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 0.0, 1.0),
            &mut enter,
            &mut exit,
            &mut enter_normal,
            &mut exit_normal,
        ) {
            return false;
        }

        let (root, outward_normal) = if enter <= t_min || t_max <= enter {
            if exit <= t_min || t_max <= exit {
                return false;
            }
            (exit, exit_normal)
        } else {
            (enter, enter_normal)
        };

        rec.t = root;
        rec.p = ray.at(rec.t);
        rec.set_face_normal(ray, outward_normal);
        true
    }
}
