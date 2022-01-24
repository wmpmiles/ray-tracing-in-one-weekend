use geometry3d::*;
use crate::random::Random;

pub struct Camera {
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Point3,
    u: Vec3,
    v: Vec3,
    lens_radius: f64,
    time_min: f64,
    time_max: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
        time_min: Option<f64>,
        time_max: Option<f64>,
    ) -> Camera {
        let theta = vfov.to_radians();
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = match (lookfrom - lookat).unit() {
            Some(vec) => vec,
            None => panic!("lookfrom and lookat cannot be the same point."),
        };
        let u = match vup.cross(w).unit() {
            Some(vec) => vec,
            None => panic!("vup and the look direction cannot be parallel."),
        };
        let v = w.cross(u);

        let origin = lookfrom;

        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;

        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

        let lens_radius = aperture / 2.0;

        let time_min = time_min.unwrap_or(0.0);
        let time_max = time_max.unwrap_or(0.0);

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

        Ray3 { origin, direction, time }
    }
}
