use crate::models::color::Color;
use crate::models::ray::Ray;
use crate::models::vec3::Vec3;
use crate::utils::random_utils::random_unit_vec;

use super::material::Material;

pub struct Lambertian {
    color: Color,
}

impl Lambertian {
    pub fn new(color: Color) -> Lambertian {
        Lambertian { color }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        ray_in: &crate::models::ray::Ray,
        hit_record: &crate::shapes::hittable::HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_dir: Vec3 = hit_record.normal() + random_unit_vec();
        if scatter_dir.near_zero() {
            scatter_dir = hit_record.normal();
        }
        *scattered = Ray::new(hit_record.p(), scatter_dir);
        *attenuation = self.color;
        true
    }

    fn my_clone(&self) -> Box<dyn Material> {
        Box::new(Lambertian::new(self.color))
    }
}
