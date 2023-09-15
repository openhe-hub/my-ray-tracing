use crate::materials::lambertian::Lambertian;
use crate::materials::material::Material;
use crate::models::color::Color;
use crate::models::ray::Ray;
use crate::models::vec3::{Point3, Vec3};
use crate::utils::interval::Interval;

pub struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f64,
    front_face: bool,
    mat: Box<dyn Material>,
}

impl HitRecord {
    pub fn new(
        p: Point3,
        normal: Vec3,
        t: f64,
        front_face: bool,
        mat: Box<dyn Material>,
    ) -> HitRecord {
        HitRecord {
            p,
            normal,
            t,
            front_face,
            mat,
        }
    }

    pub fn empty() -> HitRecord {
        HitRecord {
            p: Point3::empty(),
            normal: Vec3::empty(),
            t: 0.0,
            front_face: false,
            mat: Box::new(Lambertian::new(Color::empty())),
        }
    }

    pub fn set_face_normal(&mut self, ray: Ray, outward_normal: Vec3) {
        self.front_face = ray.dir().dot(&outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            outward_normal.scale_mul(-1.0)
        };
    }

    pub fn p(&self) -> Point3 {
        self.p
    }

    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn mat(&self) -> Box<dyn Material> {
        self.mat.my_clone()
    }

    pub fn set_p(&mut self, p: Point3) {
        self.p = p;
    }

    pub fn set_normal(&mut self, normal: Vec3) {
        self.normal = normal;
    }

    pub fn set_t(&mut self, t: f64) {
        self.t = t;
    }

    pub fn set_mat(&mut self, mat: Box<dyn Material>) {
        self.mat = mat;
    }
}

pub trait Hittable {
    fn hit(&self, ray: Ray, ray_t_interval: Interval, hit_record: &mut HitRecord) -> bool;
}
