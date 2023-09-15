use super::common_value::CONSTANT;

pub fn degree_to_rad(degree: f64) -> f64 {
    degree * CONSTANT.PI / 180.0
}
