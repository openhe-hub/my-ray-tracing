use crate::models::ray::Ray;
use crate::{
    models::vec3::{Point3, Vec3},
    utils::interval::Interval,
};

use super::hittable::{HitRecord, Hittable};

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, ray_t_interval: Interval, hit_record: &mut HitRecord) -> bool {
        let oc: Vec3 = ray.origin() - self.center;
        let a: f64 = ray.dir().length_squared();
        let half_b: f64 = oc.dot(&ray.dir());
        let c: f64 = oc.length_squared() - self.radius.powf(2.0);

        let delta: f64 = half_b.powf(2.0) - a * c;
        if delta < 0.0 {
            return false;
        }

        let sqrt_delta: f64 = delta.sqrt();
        let mut root: f64 = (-half_b - sqrt_delta) / a;
        if !ray_t_interval.surrounds(root) {
            root = (-half_b + sqrt_delta) / a;
            if !ray_t_interval.surrounds(root) {
                return false;
            }
        }

        hit_record.set_t(root);
        hit_record.set_p(ray.at(hit_record.t()));
        let outward_normal: Vec3 = (hit_record.p() - self.center).scale_mul(1.0 / self.radius);
        hit_record.set_face_normal(ray, outward_normal);
        return true;
    }
}
