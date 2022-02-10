use crate::material::*;
use geometry3d::*;

pub struct HitRecord<'a> {
    pub point: Point3,
    pub normal: Vec3,
    pub material: &'a Material,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl<'a> HitRecord<'a> {
    pub fn new(
        point: Point3,
        outward_normal: Vec3,
        ray_in: Ray3,
        material: &'a Material,
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
            material,
            t,
            u,
            v,
            front_face,
        }
    }
}
