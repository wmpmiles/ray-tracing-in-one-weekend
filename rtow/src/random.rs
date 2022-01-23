/* Random value generation
 * - random unit vector
 * - random vector in unit disk
 * - random vector in unit sphere
 */

use crate::color::*;
use geometry3d::*;
use rand::{Rng, RngCore};
use rand::distributions::{Standard, Distribution};

pub struct Random<'a>(&'a mut dyn RngCore);

impl<'a> Random<'a> {
    pub fn new(rng: &'a mut dyn RngCore) -> Self {
        Random(rng)
    }

    pub fn random<T>(&mut self) -> T 
    where
        Standard: Distribution<T>
    {
        self.0.gen()
    }

    pub fn color(&mut self) -> FloatRgb {
        FloatRgb::new(self.random(), self.random(), self.random())
    }

    pub fn in_unit_cube(&mut self) -> Vec3 {
        let v = Vec3::new(self.random(), self.random(), self.random());
        (2.0 * v) - Vec3::new(1.0, 1.0, 1.0)
    }

    pub fn in_unit_sphere(&mut self) -> Vec3 {
        loop {
            let v = self.in_unit_cube();
            if v.quadrance() <= 1.0 {
                return v;
            }
        }
    }

    pub fn unit_vector(&mut self) -> Vec3 {
        loop {
            if let Some(vec) = self.in_unit_sphere().unit() {
                return vec;
            }
        }
    }

    pub fn in_unit_disk(&mut self) -> Vec3 {
        loop {
            let v = self.in_unit_cube();
            let v = Vec3::new(v.x(), v.y(), 0.0);
            if v.quadrance() <= 1.0 {
                return v;
            }
        }
    }
}
