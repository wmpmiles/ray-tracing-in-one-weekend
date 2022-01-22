/* Random value generation
 * - random unit vector
 * - random vector in unit disk
 * - random vector in unit sphere
 */

use geometry3d::*;
use crate::color::*;
use rand::Rng;

pub fn color() -> FloatRgb {
    let mut rng = rand::thread_rng();
    FloatRgb::new(rng.gen(), rng.gen(), rng.gen())
}

pub fn in_unit_cube() -> Vec3 {
    let mut rng = rand::thread_rng();
    let v = Vec3::new(rng.gen(), rng.gen(), rng.gen());
    (2.0 * v) - Vec3::new(1.0, 1.0, 1.0)
}

pub fn in_unit_sphere() -> Vec3 {
    loop {
        let v = in_unit_cube();
        if v.quadrance() <= 1.0 {
            return v;
        }
    }
}

pub fn unit_vector() -> Vec3 {
    loop {
        if let Some(vec) = in_unit_sphere().unit() {
            return vec;
        }
    }
}

pub fn in_disk() -> Vec3 {
    loop {
        let v = in_unit_cube();
        let v = Vec3::new(v.x(), v.y(), 0.0);
        if v.quadrance() <= 1.0 {
            return v;
        }
    }
}
