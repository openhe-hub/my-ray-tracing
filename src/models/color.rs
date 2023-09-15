use std::ops::Add;

use crate::utils::interval::Interval;

use super::vec3::Vec3;

#[derive(Debug)]
pub struct Color {
    r: u32,
    g: u32,
    b: u32,
}

impl Color {
    pub fn new(r: u32, g: u32, b: u32) -> Color {
        Color { r, g, b }
    }

    pub fn empty() -> Color {
        Color { r: 0, g: 0, b: 0 }
    }

    pub fn scale_to_rgb255(r: f64, g: f64, b: f64) -> Color {
        let ir = (255.999 * r) as u32;
        let ig = (255.999 * g) as u32;
        let ib = (255.999 * b) as u32;
        Color {
            r: ir,
            g: ig,
            b: ib,
        }
    }

    pub fn scale_vec3_to_rgb255(vec: Vec3) -> Color {
        Color::scale_to_rgb255(vec.x(), vec.y(), vec.z())
    }

    pub fn sample(&mut self, samples_per_px: i32) {
        let r = self.r;
        let g = self.g;
        let b = self.b;

        let scale = 1.0 / (samples_per_px as f64);
        let dr = r as f64 * scale / 256.0;
        let dg = g as f64 * scale / 256.0;
        let db = b as f64 * scale / 256.0;

        let intensity: Interval = Interval::new(0.0, 0.999);
        self.r = (intensity.clamp(dr) * 256.0) as u32;
        self.g = (intensity.clamp(dg) * 256.0) as u32;
        self.b = (intensity.clamp(db) * 256.0) as u32;
    }

    pub fn scale_mul(&mut self, scale: f64){
        self.r = ((self.r as f64) * scale) as u32;
        self.g = ((self.g as f64) * scale) as u32;
        self.b = ((self.b as f64) * scale) as u32;
    }

    pub fn r(&self) -> u32 {
        self.r
    }
    pub fn g(&self) -> u32 {
        self.g
    }
    pub fn b(&self) -> u32 {
        self.b
    }
}

impl Add for Color {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Color {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}
