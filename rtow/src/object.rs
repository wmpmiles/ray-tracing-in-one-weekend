use crate::hit_record::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use vec3::*;

pub enum Object {
    Sphere(Sphere),
    List(List),
}

impl Object {
    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self {
            Object::Sphere(sphere) => sphere.hit(ray, t_min, t_max),
            Object::List(list) => list.hit(ray, t_min, t_max),
        }
    }
}

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Material) -> Object {
        Object::Sphere(Self {
            center,
            radius,
            material,
        })
    }

    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let half_b = ray.direction.dot(oc);
        let c = oc.dot(oc) - self.radius * self.radius;
        let delta = half_b * half_b - a * c;
        if delta < 0.0 {
            // no solutions -> no intersection
            return None;
        }

        // find the nearest root that lies in the acceptable range
        let sqrtd = delta.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let t = root;
        let point = ray.at(t);
        let outward_normal = (point - self.center).scalar_div(self.radius).unwrap();
        let material = self.material;

        Some(HitRecord::new(point, outward_normal, ray, material, t))
    }
}

pub struct List {
    objects: Vec<Object>,
}

impl List {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Object) {
        self.objects.push(object);
    }

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest: Option<HitRecord> = None;

        for object in &self.objects {
            let t_max = match &closest {
                None => t_max,
                Some(hit) => hit.t,
            };

            if let Some(hit) = object.hit(ray, t_min, t_max) {
                closest = Some(hit);
            }
        }

        closest
    }
}
