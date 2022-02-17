use crate::color::FloatRgb;
use crate::hit_record::HitRecord;
use crate::perlin::Perlin;
use geometry3d::*;
use ntuple::NTuple;
use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Texture {
    SolidColor(SolidColor),
    CheckerTexture(CheckerTexture),
    NoiseTexture(NoiseTexture),
    ImageTexture(ImageTexture),
}

impl Texture {
    pub fn value(&mut self, rec: HitRecord) -> FloatRgb {
        match self {
            Texture::SolidColor(t) => t.value(rec),
            Texture::CheckerTexture(t) => t.value(rec),
            Texture::NoiseTexture(t) => t.value(rec),
            Texture::ImageTexture(t) => t.value(rec),
        }
    }
}

impl From<FloatRgb> for Texture {
    fn from(frgb: FloatRgb) -> Texture {
        let t: SolidColor = frgb.into();
        Texture::SolidColor(t)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SolidColor(FloatRgb);

impl SolidColor {
    pub fn new(r: f64, g: f64, b: f64) -> SolidColor {
        SolidColor(FloatRgb::new(r, g, b))
    }

    fn value(&self, _rec: HitRecord) -> FloatRgb {
        self.0
    }
}

impl From<FloatRgb> for SolidColor {
    fn from(frgb: FloatRgb) -> SolidColor {
        SolidColor(frgb)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckerTexture {
    odd: Box<Texture>,
    even: Box<Texture>,
}

impl CheckerTexture {
    pub fn new(odd: Texture, even: Texture) -> CheckerTexture {
        let (odd, even) = (Box::new(odd), Box::new(even));
        CheckerTexture { odd, even }
    }

    fn value(&mut self, rec: HitRecord) -> FloatRgb {
        let p = rec.point;
        let sines = (10.0 * p.x()).sin() * (10.0 * p.y()).sin() * (10.0 * p.z()).sin();
        let t = if sines < 0.0 {
            &mut self.odd
        } else {
            &mut self.even
        };
        t.value(rec)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
    depth: usize,
}

impl NoiseTexture {
    pub fn new(noise: Perlin, scale: f64, depth: usize) -> NoiseTexture {
        NoiseTexture {
            noise,
            scale,
            depth,
        }
    }

    pub fn value(&mut self, rec: HitRecord) -> FloatRgb {
        let white = FloatRgb::new(1.0, 1.0, 1.0);
        let black = FloatRgb::new(0.0, 0.0, 0.0);
        let point = Point3::from(self.scale * Vec3::from(rec.point));
        let noise = 0.5
            * (1.0
                + f64::sin(
                    self.scale * point.z() + 10.0 * self.noise.turbulence(point, self.depth),
                ));
        white.mix(black, noise)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImageTexture {
    U(ImageTextureUninit),
    I(Option<ImageTextureInit>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageTextureUninit {
    filename: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageTextureInit {
    width: usize,
    height: usize,
    bytes_per_row: usize,
    data: Vec<u8>,
}

impl ImageTexture {
    const BYTES_PER_PIXEL: usize = 3;

    pub fn new(filename: String) -> ImageTexture {
        let inner = ImageTextureUninit { filename };
        ImageTexture::U(inner)
    }

    pub fn value(&mut self, rec: HitRecord) -> FloatRgb {
        if let ImageTexture::U(u) = self {
            *self = ImageTexture::I(Self::init(u));
        }

        if let ImageTexture::I(i) = self {
            Self::value_calc(i, rec)
        } else {
            panic!("Failed to initialise ImageTexture.");
        }
    }

    fn init(u: &ImageTextureUninit) -> Option<ImageTextureInit> {
        let file = File::open(&u.filename);
        if let Ok(file) = file {
            let decoder = png::Decoder::new(file);

            let mut reader = decoder
                .read_info()
                .unwrap_or_else(|_| panic!("Failed to read info in {}", &u.filename));

            let info = reader.info();
            assert!(!info.is_animated(), "{} cannot be an APNG.", &u.filename);
            assert_eq!(
                info.bit_depth,
                png::BitDepth::Eight,
                "The bit depth of {} is not eight.",
                &u.filename
            );
            assert_eq!(
                info.color_type,
                png::ColorType::Rgb,
                "The color type of {} is not RGB.",
                &u.filename
            );
            assert!(!info.interlaced, "{} cannot be interlaced.", &u.filename);

            let mut data = vec![0; reader.output_buffer_size()];
            let output_info = reader
                .next_frame(&mut data)
                .unwrap_or_else(|_| panic!("Failed to decode frame data in {}", &u.filename));

            let width = output_info.width as usize;
            let height = output_info.height as usize;
            let bytes_per_row = output_info.line_size;

            Some(ImageTextureInit { width, height, bytes_per_row, data })
        } else {
            None
        }
    }

    fn value_calc(s: &Option<ImageTextureInit>, rec: HitRecord) -> FloatRgb {
        const COLOR_SCALE: f64 = 1.0 / 255.0;

        if let Some(it) = s {
            let u = rec.u.clamp(0.0, 1.0);
            let v = 1.0 - rec.v.clamp(0.0, 1.0);

            let i = ((u * (it.width as f64)) as usize).clamp(0, it.width - 1);
            let j = ((v * (it.height as f64)) as usize).clamp(0, it.height - 1);

            let start = j * it.bytes_per_row + i * Self::BYTES_PER_PIXEL;
            let stop = start + Self::BYTES_PER_PIXEL;

            let color_tuple = NTuple::from(&it.data[start..stop]).map(|x| COLOR_SCALE * (x as f64));
            FloatRgb::from(color_tuple)
        } else {
            // Empty image textures rendered as cyan
            FloatRgb::new(0.0, 1.0, 1.0)
        }
    }
}
