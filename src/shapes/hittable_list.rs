use crate::models::ray::Ray;

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

    pub fn hit(&self, ray: Ray, ray_tmin: f64, ray_tmax: f64, hit_record: &mut HitRecord) -> bool {
        let mut hit_anything: bool = false;
        let mut closest_t: f64 = ray_tmax;
        // let mut tmp_record: HitRecord = HitRecord::empty();

        for object in self.objects.iter() {
            if object.hit(ray, ray_tmin, closest_t, hit_record){
                hit_anything = true;
                closest_t = hit_record.t();
            }
        }

        hit_anything
    }
}
