mod models;

use models::color::Color;
use models::image::Image;

fn main() -> std::io::Result<()> {
    let mut img: Image = Image::new(256, 256);
    for i in 0..img.height() {
        for j in 0..img.width() {
            let r: f32 = i as f32 / (img.width() - 1) as f32;
            let g: f32 = j as f32 / (img.height() - 1) as f32;
            let b: f32 = 0.0;
            let color: Color = Color::scale_to_rgb255(r, g, b);
            img.set_color(color, i, j);
        }
    }
    img.export()?;
    Ok(())
}
