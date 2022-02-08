use crate::color::FloatRgb;
use crate::hit_record::HitRecord;

pub trait Texture: CloneTexture {
    fn value(&self, rec: &HitRecord) -> FloatRgb;
}

pub trait CloneTexture {
    fn clone_texture(&self) -> Box<dyn Texture>;
}

impl<T> CloneTexture for T
where
    T: Texture + Clone + 'static,
{
    fn clone_texture(&self) -> Box<dyn Texture> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Texture> {
    fn clone(&self) -> Box<dyn Texture> {
        self.clone_texture()
    }
}

impl From<FloatRgb> for Box<dyn Texture> {
    fn from(frgb: FloatRgb) -> Box<dyn Texture> {
        let t: SolidColor = frgb.into();
        Box::new(t)
    }
}

#[derive(Clone, Copy)]
pub struct SolidColor(FloatRgb);

impl SolidColor {
    pub fn new(r: f64, g: f64, b: f64) -> SolidColor {
        SolidColor(FloatRgb::new(r, g, b))
    }
}

impl From<FloatRgb> for SolidColor {
    fn from(frgb: FloatRgb) -> SolidColor {
        SolidColor(frgb)
    }
}

impl Texture for SolidColor {
    fn value(&self, _rec: &HitRecord) -> FloatRgb {
        self.0
    }
}

#[derive(Clone)]
pub struct CheckerTexture {
    odd: Box<dyn Texture>,
    even: Box<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(odd: Box<dyn Texture>, even: Box<dyn Texture>) -> CheckerTexture {
        CheckerTexture { odd, even }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, rec: &HitRecord) -> FloatRgb {
        let p = rec.point;
        let sines = (10.0 * p.x()).sin() * (10.0 * p.y()).sin() * (10.0 * p.z()).sin();
        let t = if sines < 0.0 { &self.odd } else { &self.even };
        t.value(rec)
    }
}
