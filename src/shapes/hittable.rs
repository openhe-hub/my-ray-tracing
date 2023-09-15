use crate::models::ray::Ray;
use crate::models::vec3::{Point3, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f64,
}

impl HitRecord {
    pub fn new(p: Point3, normal: Vec3, t: f64) -> HitRecord {
        HitRecord { p, normal, t }
    }

    pub fn p(&self) -> Point3 {
        self.p
    }

    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn set_p(&mut self, p: Point3) {
        self.p = p;
    }

    pub fn set_normal(&mut self, normal: Vec3) {
        self.normal = normal;
    }

    pub fn set_t(&mut self, t: f64) {
        self.t = t;
    }
}

pub trait Hittable {
    pub fn hit(&self, ray: Ray, ray_tmin: f64, ray_tmax: f64, hit_record: &mut HitRecord) -> bool;
}
