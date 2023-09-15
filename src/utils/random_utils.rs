use rand::Rng;

pub fn random_f64() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

pub fn random_f64_ranged(fmin: f64, fmax: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(fmin..fmax)
}
