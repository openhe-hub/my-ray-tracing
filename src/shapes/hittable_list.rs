use crate::{models::ray::Ray, utils::interval::Interval};

use super::hittable::{HitRecord, Hittable};

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList { objects: vec![] }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn hit(&self, ray: Ray, ray_t_interval: Interval, hit_record: &mut HitRecord) -> bool {
        let mut hit_anything: bool = false;
        let mut closest_t: f64 = ray_t_interval.max();
        // let mut tmp_record: HitRecord = HitRecord::empty();

        for object in self.objects.iter() {
            if object.hit(ray, Interval::new(ray_t_interval.min(), closest_t), hit_record) {
                hit_anything = true;
                closest_t = hit_record.t();
            }
        }

        hit_anything
    }
}
