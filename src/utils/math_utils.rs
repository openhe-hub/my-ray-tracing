use crate::models::vec3::Vec3;

use super::common_value::CONSTANT;

pub fn degree_to_rad(degree: f64) -> f64 {
    degree * CONSTANT.PI / 180.0
}

pub fn linear_to_gamma(linear_val: f64) -> f64 {
    linear_val.sqrt()
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - n.scale_mul((v.dot(n)) * 2.0)
}
