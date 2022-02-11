use geometry3d::*;

#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub ray_in: Ray3,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(
        point: Point3,
        outward_normal: Vec3,
        ray_in: Ray3,
        t: f64,
        u: f64,
        v: f64,
    ) -> HitRecord {
        let front_face = ray_in.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        HitRecord {
            point,
            normal,
            ray_in,
            t,
            u,
            v,
            front_face,
        }
    }
}
