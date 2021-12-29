use crate::ray::Ray;
use crate::vec3::*;

pub struct Camera {
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Point3,
}

impl Camera {
    pub const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = Self::ASPECT_RATIO * Self::VIEWPORT_HEIGHT;
    const FOCAL_LENGTH: f64 = 1.0;

    pub fn new() -> Camera {
        let origin = Point3::new();
        let horizontal = Vec3::from(Self::VIEWPORT_WIDTH, 0.0, 0.0);
        let vertical = Vec3::from(0.0, Self::VIEWPORT_HEIGHT, 0.0);
        let lower_left_corner = origin
            - horizontal.scalar_div(2.0)
            - vertical.scalar_div(2.0)
            - Vec3::from(0.0, 0.0, Self::FOCAL_LENGTH);

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner
                + self.horizontal.scalar_mul(u)
                + self.vertical.scalar_mul(v)
                - self.origin,
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new()
    }
}
