mod models;
mod shapes;
mod utils;
mod materials;

extern crate rand;

use materials::lambertian::Lambertian;
use materials::material::Material;
use materials::metal::Metal;
use models::camera::Camera;
use models::color::Color;
use models::vec3::Point3;
use shapes::hittable_list::HittableList;
use shapes::sphere::Sphere;

fn main() -> std::io::Result<()> {
    // camera
    let mut camera: Camera = Camera::new();
    camera.init();

    // material
    let material_ground: Box<dyn Material> = Box::new(Lambertian::new(Color::scale_to_rgb255(0.8, 0.8, 0.0)));
    let material_center: Box<dyn Material> = Box::new(Lambertian::new(Color::scale_to_rgb255(0.7, 0.3, 0.3)));
    let material_left: Box<dyn Material> = Box::new(Metal::new(Color::scale_to_rgb255(0.8, 0.8, 0.8)));
    let material_right: Box<dyn Material> = Box::new(Metal::new(Color::scale_to_rgb255(0.8, 0.6, 0.2)));

    // world
    let mut world: HittableList = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, material_center)));
    world.add(Box::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_left)));
    world.add(Box::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_right)));


    // render
    camera.render(&world)?;
    Ok(())
}
