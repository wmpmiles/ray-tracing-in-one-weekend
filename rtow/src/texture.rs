use crate::color::FloatRgb;
use crate::hit_record::HitRecord;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Texture {
    SolidColor(SolidColor),
    CheckerTexture(CheckerTexture),
}

impl Texture {
    pub fn value(&self, rec: &HitRecord) -> FloatRgb {
        match self {
            Texture::SolidColor(t) => t.value(rec),
            Texture::CheckerTexture(t) => t.value(rec),
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

    fn value(&self, _rec: &HitRecord) -> FloatRgb {
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

    fn value(&self, rec: &HitRecord) -> FloatRgb {
        let p = rec.point;
        let sines = (10.0 * p.x()).sin() * (10.0 * p.y()).sin() * (10.0 * p.z()).sin();
        let t = if sines < 0.0 { &self.odd } else { &self.even };
        t.value(rec)
    }
}

