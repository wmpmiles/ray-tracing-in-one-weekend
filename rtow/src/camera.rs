use crate::random::Random;
use geometry3d::*;
use crate::config::*;

pub struct Camera {
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Point3,
    u: Vec3,
    v: Vec3,
    lens_radius: f64,
    pub time_min: f64,
    pub time_max: f64,
}

impl Camera {
    pub fn new(config: CameraConfig) -> Camera {
        let theta = config.vertical_fov.to_radians();
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = config.aspect_ratio * viewport_height;

        let w = match (config.look_from - config.look_at).unit() {
            Some(vec) => vec,
            None => panic!("lookfrom and lookat cannot be the same point."),
        };
        let u = match config.up.cross(w).unit() {
            Some(vec) => vec,
            None => panic!("vup and the look direction cannot be parallel."),
        };
        let v = w.cross(u);

        let origin = config.look_from;

        let horizontal = config.focus_distance * viewport_width * u;
        let vertical = config.focus_distance * viewport_height * v;

        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - config.focus_distance * w;

        let lens_radius = config.aperture / 2.0;

        let time_min = config.time_min;
        let time_max = config.time_max;

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            lens_radius,
            time_min,
            time_max,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray3 {
        let mut rng = Random::new(rand::thread_rng());
        let rd = self.lens_radius * rng.in_unit_disk();
        let offset = rd.x() * self.u + rd.y() * self.v;

        let origin = self.origin + offset;
        let direction = (self.lower_left_corner + s * self.horizontal + t * self.vertical
            - self.origin
            - offset)
            .unit()
            .unwrap();
        let time = rng.random_range(self.time_min..=self.time_max);

        Ray3 {
            origin,
            direction,
            time,
        }
    }
}
