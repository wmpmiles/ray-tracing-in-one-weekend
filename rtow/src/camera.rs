use crate::ray::Ray;
use vec3::*;

pub struct Camera {
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Point3,
    u: Vec3,
    v: Vec3,
    lens_radius: f64,
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
    ) -> Camera {
        let theta = vfov.to_radians();
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = Vec3::unit_vector(lookfrom - lookat).unwrap();
        let u = Vec3::unit_vector(Vec3::cross(vup, w)).unwrap();
        let v = Vec3::cross(w, u);

        let origin = lookfrom;

        let horizontal = u.scalar_mul(focus_dist * viewport_width);
        let vertical = v.scalar_mul(focus_dist * viewport_height);

        let scalar2 = Vec3::scalar(2.0);
        let lower_left_corner =
            origin - horizontal / scalar2 - vertical / scalar2 - w.scalar_mul(focus_dist);

        let lens_radius = aperture / 2.0;

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            lens_radius,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = Vec3::random_in_unit_disk().scalar_mul(self.lens_radius);
        let offset = self.u.scalar_mul(rd.0) + self.v.scalar_mul(rd.1);

        let origin = self.origin + offset;
        let direction =
            (self.lower_left_corner + self.horizontal.scalar_mul(s) + self.vertical.scalar_mul(t)
                - self.origin
                - offset)
                .unit_vector()
                .unwrap();

        Ray { origin, direction }
    }
}
