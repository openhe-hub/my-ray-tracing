use crate::{
    models::{color::Color, ray::Ray, vec3::Vec3},
    utils::math_utils::reflect,
};

use super::material::Material;

pub struct Metal {
    color: Color,
}

impl Metal {
    pub fn new() -> Metal {
        Metal {
            color: Color::empty(),
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray_in: &crate::models::ray::Ray,
        hit_record: &crate::shapes::hittable::HitRecord,
        attenuation: &mut Color,
        scattered: &mut crate::models::ray::Ray,
    ) -> bool {
        let reflected: Vec3 = reflect(&ray_in.dir(), &hit_record.normal());
        *scattered = Ray::new(hit_record.p(), reflected);
        *attenuation = self.color;
        true
    }
}
