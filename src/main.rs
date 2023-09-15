mod models;
mod shapes;
mod utils;

extern crate rand;

use models::camera::Camera;
use models::vec3::Point3;
use shapes::hittable_list::HittableList;
use shapes::sphere::Sphere;

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
