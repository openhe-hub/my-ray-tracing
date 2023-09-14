mod models;

use models::color::Color;
use models::image::Image;

fn main() -> std::io::Result<()> {
    let mut img: Image = Image::new(256, 256);
    for i in 0..img.get_height() {
        for j in 0..img.get_width() {
            let r: f32 = i as f32 / (img.get_width() - 1) as f32;
            let g: f32 = j as f32 / (img.get_height() - 1) as f32;
            let b: f32 = 0.0;
            let ir = (255.999 * r) as u32;
            let ig = (255.999 * g) as u32;
            let ib = (255.999 * b) as u32;
            let color: Color = Color::new(ir, ig, ib);
            img.set_color(color, i as usize, j as usize);
        }
    }
    img.export()?;
    Ok(())
}
