mod models;
mod shapes;
mod utils;

extern crate rand;

use models::camera::Camera;
use models::color::Color;
use models::image::Image;
use models::ray::Ray;
use models::vec3::{Point3, Vec3};
use shapes::hittable::HitRecord;
use shapes::hittable_list::HittableList;
use shapes::sphere::Sphere;
use utils::common_value::CONSTANT;
use utils::interval::Interval;

fn ray_color(ray: Ray, world: &HittableList) -> Color {
    let mut hit_record: HitRecord = HitRecord::empty();
    if world.hit(ray, Interval::new(0.0, CONSTANT.INFINITY), &mut hit_record) {
        let color_vec: Vec3 = (hit_record.normal() + Vec3::new(1.0, 1.0, 1.0)).scale_mul(0.5);
        return Color::scale_vec3_to_rgb255(color_vec);
    }

    let unit_dir: Vec3 = ray.dir().unit();
    let a: f64 = 0.5 * (unit_dir.y() + 1.0);
    let color_vec: Vec3 =
        Vec3::new(1.0, 1.0, 1.0).scale_mul(1.0 - a) + Vec3::new(0.5, 0.7, 1.0).scale_mul(a);
    return Color::scale_vec3_to_rgb255(color_vec);
}

fn main() -> std::io::Result<()> {
    // camera
    let mut camera: Camera = Camera::new();
    camera.init();

    // world
    let mut world: HittableList = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // render
    camera.render(&world)?;
    Ok(())
}
