use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use super::color::Color;

#[derive(Debug)]
pub struct Image {
    width: u32,
    height: u32,
    content: Vec<Vec<Color>>,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Image {
        let mut content: Vec<Vec<Color>> = Vec::new();
        for i in 0..height {
            content.push(vec![]);
            for _ in 0..width {
                content[i as usize].push(Color::empty());
            }
        }
        Image {
            width,
            height,
            content,
        }
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn set_color(&mut self, color: Color, pos_x: usize, pos_y: usize) {
        self.content[pos_x][pos_y] = color;
    }

    pub fn export(&self) -> std::io::Result<()> {
        let path = Path::new("assets/output.ppm");
        let mut file = File::create(&path)?;

        writeln!(file, "P3")?;
        writeln!(file, "{} {}", self.width, self.height)?;
        writeln!(file, "255")?;
        for i in 0..self.height {
            println!("{} lines left.", self.height - i);
            for j in 0..self.width {
                let curr_color = &self.content[i as usize][j as usize];
                writeln!(
                    file,
                    "{} {} {}",
                    curr_color.r(),
                    curr_color.g(),
                    curr_color.b()
                )?;
            }
        }
        Ok(())
    }
}
