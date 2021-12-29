use crate::vec3::Point3;
use crate::vec3::Vec3;

#[derive(Default)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction.scalar_mul(t)
    }
}
