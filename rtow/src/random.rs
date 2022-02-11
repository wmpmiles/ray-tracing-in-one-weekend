/* Random value generation
 * - random unit vector
 * - random vector in unit disk
 * - random vector in unit sphere
 */

use crate::color::*;
use geometry3d::*;
use rand::{Rng};
use rand::distributions::{Standard, Distribution, uniform::{SampleUniform, SampleRange}};

#[derive(Debug, Clone)]
pub struct Random<T: Rng>(T);

impl<T: Rng> Random<T> {
    pub fn new(rng: T) -> Self {
        Random(rng)
    }

    pub fn random<S>(&mut self) -> S 
    where
        Standard: Distribution<S>
    {
        self.0.gen()
    }

    pub fn random_range<S, R>(&mut self, range: R) -> S
    where
        S: SampleUniform,
        R: SampleRange<S>,
    {
        self.0.gen_range(range)
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
