mod models;
mod shapes;
mod utils;

use models::color::Color;
use models::image::Image;
use models::ray::Ray;
use models::vec3::{Point3, Vec3};
use shapes::hittable::{HitRecord, Hittable};
use shapes::hittable_list::HittableList;
use shapes::sphere::Sphere;
use utils::common_value::CONSTANT;

fn ray_color(ray: Ray, world: &HittableList) -> Color {
    let mut hit_record: HitRecord = HitRecord::empty();
    if world.hit(ray, 0.0, CONSTANT.INFINITY, &mut hit_record) {
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
    // image
    let aspect_ratio = 16.0 / 9.0;
    const image_width: u32 = 400;

    // calc the img height
    let mut image_height = (image_width as f64 / aspect_ratio) as u32;
    image_height = image_height.max(1);

    // world
    let mut world: HittableList = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // camera
    const focal_length: f64 = 1.0;
    const viewport_height: f64 = 2.0;
    let viewport_width: f64 = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center: Point3 = Point3::new(0.0, 0.0, 0.0);

    // calc viewport vec
    let viewport_u: Vec3 = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v: Vec3 = Vec3::new(0.0, -viewport_height, 0.0);

    // calc viewport vec per px
    let px_delta_u: Vec3 = viewport_u.scale_mul(1.0 / image_width as f64);
    let px_delta_v: Vec3 = viewport_v.scale_mul(1.0 / image_height as f64);

    // calc the loc of px00
    let viewport_00: Point3 = camera_center
        - Vec3::new(0.0, 0.0, focal_length)
        - viewport_u.scale_mul(0.5)
        - viewport_v.scale_mul(0.5);
    let px_00: Point3 = viewport_00 + (px_delta_u + px_delta_v).scale_mul(0.5);

    let mut img: Image = Image::new(image_width, image_height);
    for i in 0..img.height() {
        for j in 0..img.width() {
            let px_center: Point3 =
                px_00 + px_delta_u.scale_mul(j as f64) + px_delta_v.scale_mul(i as f64);
            let ray_dir: Vec3 = px_center - camera_center;
            let ray: Ray = Ray::new(camera_center, ray_dir);
            img.set_color(ray_color(ray, &world), i, j);
        }
    }
    img.export()?;
    Ok(())
}
