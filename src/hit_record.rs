use crate::material::*;
use crate::ray::Ray;
use crate::vec3::*;
use std::rc::Rc;

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub material: Rc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            material: Rc::new(Lambertian {
                albedo: Default::default(),
            }),
            point: Default::default(),
            normal: Default::default(),
            t: Default::default(),
            front_face: Default::default(),
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

impl Default for HitRecord {
    fn default() -> Self {
        Self::new()
    }
}
