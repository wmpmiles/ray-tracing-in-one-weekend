use crate::color::*;
use crate::config::ImageConfig;

pub struct Image {
    pub filename: String,
    pub width: u32,
    pub height: u32,
    pub aspect_ratio: f64,
    data: Vec<u8>,
}

pub struct ImageIter {
    width: u32,
    x: u32,
    y: i32,
}

impl Image {
    pub fn new(config: ImageConfig) -> Image {
        let filename = config.filename;
        let width = config.width;
        let height = config.height;
        let aspect_ratio = width as f64 / height as f64;
        let size = (width * height * 3) as usize;
        let data = Vec::with_capacity(size);
        Image {
            filename,
            width,
            height,
            aspect_ratio,
            data,
        }
    }

    pub fn iter(&self) -> ImageIter {
        ImageIter {
            width: self.width,
            x: 0,
            y: (self.height - 1) as i32,
        }
    }

    pub fn write(&self) -> std::io::Result<()> {
        use std::fs::File;
        use std::io::BufWriter;
        use std::path::Path;

        let path = Path::new(&self.filename);
        let file = File::create(path)?;
        let w = BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, self.width, self.height);
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);

        let mut writer = encoder.write_header()?;
        writer.write_image_data(&self.data)?;

        Ok(())
    }

    pub fn add_pixel(&mut self, color: Rgb) {
        self.data.push(color.r());
        self.data.push(color.g());
        self.data.push(color.b());
    }
}

impl Iterator for ImageIter {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.y < 0 {
            None
        } else {
            let ret = Some(( self.x, self.y as u32 ));

            self.x = (self.x + 1) % self.width;
            if self.x == 0 {
                eprint!("\rScanlines remaining: {} ", self.y);
                self.y -= 1;
            }

            ret
        }
    }
}
