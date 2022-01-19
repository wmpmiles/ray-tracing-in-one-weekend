use crate::vec3::Color;

pub struct Image {
    pub aspect_ratio: f64,
    pub width: u32,
    pub height: u32,
    data: Vec<u8>,
}

pub struct ImageIter {
    width: u32,
    x: u32,
    y: i32,
}

impl Image {
    pub fn new(aspect_ratio: f64, width: u32) -> Image {
        let height = (width as f64 / aspect_ratio) as u32;
        let size = (width * height * 3) as usize;
        let data = Vec::with_capacity(size);
        Image {
            aspect_ratio,
            width,
            height,
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

    pub fn write(&self, filename: &str) -> std::io::Result<()> {
        use std::fs::File;
        use std::io::BufWriter;
        use std::path::Path;

        let path = Path::new(filename);
        let file = File::create(path)?;
        let w = BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, self.width, self.height);
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);

        let mut writer = encoder.write_header()?;
        writer.write_image_data(&self.data)?;

        Ok(())
    }

    fn map_byte(val: f64) -> u8 {
        let val = val.clamp(0.0, 1.0);
        const RATIO: f64 = 255.999;
        (RATIO * val) as u8
    }

    pub fn add_pixel(&mut self, color: Color, samples: u32) {
        let scale = 1.0 / samples as f64;
        let color = color.elementwise(|x| (scale * x).sqrt());

        self.data.push(Self::map_byte(color.0));
        self.data.push(Self::map_byte(color.1));
        self.data.push(Self::map_byte(color.2));
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
