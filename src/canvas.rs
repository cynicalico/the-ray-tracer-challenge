use crate::color::Color;
use std::fs::File;
use std::io::BufWriter;
use std::ops::{Index, IndexMut};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![Color::new(0.0, 0.0, 0.0); width * height],
        }
    }

    pub fn save_png(self: &Self, path: PathBuf) -> Result<(), std::io::Error> {
        let path = path.as_path();
        let file = File::create(path)?;
        let ref mut w = BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, self.width as u32, self.height as u32);
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header()?;

        let data = self
            .pixels
            .iter()
            .map(|c| {
                vec![
                    (c.r * 255.0).round().clamp(0.0, 255.0) as u8,
                    (c.g * 255.0).round().clamp(0.0, 255.0) as u8,
                    (c.b * 255.0).round().clamp(0.0, 255.0) as u8,
                ]
            })
            .flatten()
            .collect::<Vec<u8>>();
        writer.write_image_data(data.as_slice())?;

        Ok(())
    }
}

impl Index<(usize, usize)> for Canvas {
    type Output = Color;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.pixels[index.1 * self.width + index.0]
    }
}

impl IndexMut<(usize, usize)> for Canvas {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.pixels[index.1 * self.width + index.0]
    }
}
