use super::color::Color;
use super::vec3::{Point3, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    origin: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, dir: Vec3) -> Ray {
        Ray { origin, dir }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn dir(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.dir.scale_mul(t)
    }

    pub fn ray_color(&self) -> Color {
        let unit_dir: Vec3 = self.dir.unit();
        let a: f64 = 0.5 * (unit_dir.y() + 1.0);
        let scaled_color: Vec3 =
            Vec3::new(1.0, 1.0, 1.0).scale_mul(1.0 - a) + Vec3::new(0.5, 0.7, 1.0).scale_mul(a);
        Color::scale_vec3_to_rgb255(scaled_color)
    }
}
