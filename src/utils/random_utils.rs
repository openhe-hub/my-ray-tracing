use rand::Rng;

use crate::models::vec3::Vec3;

pub fn random_f64() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

pub fn random_f64_ranged(fmin: f64, fmax: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(fmin..fmax)
}

pub fn random_vec() -> Vec3 {
    Vec3::new(random_f64(), random_f64(), random_f64())
}

pub fn random_vec_ranged(fmin: f64, fmax: f64) -> Vec3 {
    Vec3::new(
        random_f64_ranged(fmin, fmax),
        random_f64_ranged(fmin, fmax),
        random_f64_ranged(fmin, fmax),
    )
}

pub fn random_vec_in_unit_sphere() -> Vec3 {
    loop {
        let p: Vec3 = random_vec_ranged(-1.0, 1.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn random_unit_vec() -> Vec3 {
    return random_vec_in_unit_sphere().unit();
}

pub fn random_vec_on_hemisphere(normal: &Vec3) -> Vec3 {
    let on_unit_sphere: Vec3 = random_unit_vec();
    if on_unit_sphere.dot(normal) > 0.0 {
        return on_unit_sphere;
    } else {
        return on_unit_sphere.scale_mul(-1.0);
    }
}
