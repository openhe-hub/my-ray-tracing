use crate::models::color::Color;
use crate::models::ray::Ray;
use crate::shapes::hittable::HitRecord;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}
