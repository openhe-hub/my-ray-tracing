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
