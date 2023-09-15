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

    pub fn hit_sphere(&self, center: Point3, radius: f64) -> f64 {
        let center_vec = self.origin - center;
        let a = self.dir().dot(&self.dir());
        let b = center_vec.dot(&self.dir()) * 2.0;
        let c = center_vec.dot(&center_vec) - radius * radius;
        let delta = b * b - 4.0 * a * c;
        if delta < 0.0 {
            return -1.0;
        } else {
            return (-b - delta.sqrt()) / (2.0 * a);
        }
    }

    pub fn ray_color(&self) -> Color {
        let t = self.hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5);
        if t > 0.0 {
            let normal_vec = (self.at(t) - Vec3::new(0.0, 0.0, -1.0)).unit();
            return Color::scale_to_rgb255(
                normal_vec.x() * 0.5 + 0.5,
                normal_vec.y() * 0.5 + 0.5,
                normal_vec.z() * 0.5 + 0.5,
            );
        }
        let unit_dir: Vec3 = self.dir.unit();
        let a: f64 = 0.5 * (unit_dir.y() + 1.0);
        let scaled_color: Vec3 =
            Vec3::new(1.0, 1.0, 1.0).scale_mul(1.0 - a) + Vec3::new(0.5, 0.7, 1.0).scale_mul(a);
        Color::scale_vec3_to_rgb255(scaled_color)
    }
}
