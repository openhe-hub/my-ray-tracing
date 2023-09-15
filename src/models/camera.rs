use crate::{
    models::color::Color,
    models::image::Image,
    models::ray::Ray,
    models::vec3::{Point3, Vec3},
    shapes::hittable::HitRecord,
    shapes::hittable_list::HittableList,
    shapes::sphere::Sphere,
    utils::common_value::CONSTANT,
    utils::random_utils::random_f64,
    utils::{interval::Interval, random_utils::random_vec_on_hemisphere},
};

pub struct Camera {
    aspect_ratio: f64,
    image_width: u32,
    image_height: u32,
    px_delta_u: Vec3,
    px_delta_v: Vec3,
    px_00: Point3,
    camera_center: Point3,
    samples_per_px: i32,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            aspect_ratio: 0.0,
            image_width: 0,
            image_height: 0,
            px_delta_u: Vec3::empty(),
            px_delta_v: Vec3::empty(),
            px_00: Point3::empty(),
            camera_center: Point3::empty(),
            samples_per_px: 0,
        }
    }

    pub fn init(&mut self) {
        self.aspect_ratio = 16.0 / 9.0;
        self.image_width = 400;

        // calc the img height
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as u32;
        self.image_height = self.image_height.max(1);

        // world
        let mut world: HittableList = HittableList::new();
        world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
        world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

        // camera
        const focal_length: f64 = 1.0;
        const viewport_height: f64 = 2.0;
        let viewport_width: f64 =
            viewport_height * (self.image_width as f64 / self.image_height as f64);
        self.camera_center = Point3::new(0.0, 0.0, 0.0);

        // calc viewport vec
        let viewport_u: Vec3 = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v: Vec3 = Vec3::new(0.0, -viewport_height, 0.0);

        // calc viewport vec per px
        self.px_delta_u = viewport_u.scale_mul(1.0 / self.image_width as f64);
        self.px_delta_v = viewport_v.scale_mul(1.0 / self.image_height as f64);

        // calc the loc of px00
        let viewport_00: Point3 = self.camera_center
            - Vec3::new(0.0, 0.0, focal_length)
            - viewport_u.scale_mul(0.5)
            - viewport_v.scale_mul(0.5);
        self.px_00 = viewport_00 + (self.px_delta_u + self.px_delta_v).scale_mul(0.5);

        // sample
        self.samples_per_px = 100;
    }

    pub fn render(&self, world: &HittableList) -> std::io::Result<()> {
        let mut img: Image = Image::new(self.image_width, self.image_height);
        for i in 0..img.height() {
            for j in 0..img.width() {
                let mut px_color: Color = Color::new(0, 0, 0);
                for sample in 0..self.samples_per_px {
                    let ray: Ray = self.get_ray(i, j);
                    px_color = px_color + self.ray_color(ray, world);
                }
                px_color.sample(self.samples_per_px);
                img.set_color(px_color, i, j);
            }
        }
        img.export()?;
        Ok(())
    }

    fn ray_color(&self, ray: Ray, world: &HittableList) -> Color {
        let mut hit_record: HitRecord = HitRecord::empty();
        if world.hit(ray, Interval::new(0.0, CONSTANT.INFINITY), &mut hit_record) {
            let dir: Vec3 = random_vec_on_hemisphere(&hit_record.normal());
            let mut next_ray_color: Color = self.ray_color(Ray::new(hit_record.p(), dir), world);
            next_ray_color.scale_mul(0.5);
            return next_ray_color;
        }

        let unit_dir: Vec3 = ray.dir().unit();
        let a: f64 = 0.5 * (unit_dir.y() + 1.0);
        let color_vec: Vec3 =
            Vec3::new(1.0, 1.0, 1.0).scale_mul(1.0 - a) + Vec3::new(0.5, 0.7, 1.0).scale_mul(a);
        return Color::scale_vec3_to_rgb255(color_vec);
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let px_center: Point3 =
            self.px_00 + self.px_delta_u.scale_mul(j as f64) + self.px_delta_v.scale_mul(i as f64);
        let px_sample = px_center + self.px_sample_square();
        let ray_origin = self.camera_center;
        let ray_dir = px_sample - ray_origin;
        return Ray::new(ray_origin, ray_dir);
    }

    fn px_sample_square(&self) -> Vec3 {
        let px = -0.5 + random_f64();
        let py = -0.5 + random_f64();
        self.px_delta_u.scale_mul(px) + self.px_delta_v.scale_mul(py)
    }
}
