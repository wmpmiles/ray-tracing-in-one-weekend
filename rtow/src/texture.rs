use crate::color::FloatRgb;
use crate::hit_record::HitRecord;
use crate::perlin::Perlin;
use geometry3d::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Texture {
    SolidColor(SolidColor),
    CheckerTexture(CheckerTexture),
    NoiseTexture(NoiseTexture),
}

impl Texture {
    pub fn value(&mut self, rec: HitRecord) -> FloatRgb {
        match self {
            Texture::SolidColor(t) => t.value(rec),
            Texture::CheckerTexture(t) => t.value(rec),
            Texture::NoiseTexture(t) => t.value(rec),
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
