use crate::hit_record::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::*;
use std::rc::Rc;

pub trait Object {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

pub struct Sphere {
    centre: Point3,
    radius: f64,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn from(centre: Point3, radius: f64, material: Rc<dyn Material>) -> Option<Sphere> {
        let zero = radius == 0.0;
        match zero {
            true => None,
            false => Some(Sphere {
                centre,
                radius,
                material,
            }),
        }
    }
}

impl Object for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = ray.origin - self.centre;
        let a = ray.direction.dot(ray.direction);
        let half_b = ray.direction.dot(oc);
        let c = oc.dot(oc) - self.radius * self.radius;
        let delta = half_b * half_b - a * c;
        if delta < 0.0 {
            // no solutions -> no intersection
            return false;
        }

        // find the nearest root that lies in the acceptable range
        let sqrtd = delta.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.point = ray.at(rec.t);
        let outward_normal = (rec.point - self.centre).scalar_div(self.radius).unwrap();
        rec.set_face_normal(ray, outward_normal);
        rec.material = Rc::clone(&self.material);

        true
    }
}

pub struct ObjectList {
    pub objects: Vec<Rc<dyn Object>>,
}

impl ObjectList {
    pub fn new() -> ObjectList {
        ObjectList {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Rc<dyn Object>) {
        self.objects.push(object);
    }
}

impl Object for ObjectList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if object.hit(ray, t_min, closest_so_far, rec) {
                hit_anything = true;
                closest_so_far = rec.t;
            }
        }

        hit_anything
    }
}

impl Default for ObjectList {
    fn default() -> Self {
        Self::new()
    }
}
