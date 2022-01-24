use crate::hit_record::HitRecord;
use crate::material::Material;
use geometry3d::*;

pub enum Object {
    Sphere(Sphere),
    List(List),
}

impl Object {
    #[inline(always)]
    pub fn hit(&self, ray: Ray3, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self {
            Object::Sphere(sphere) => sphere.hit(ray, t_min, t_max),
            Object::List(list) => list.hit(ray, t_min, t_max),
        }
    }
}

pub enum Location {
    Ray(Ray3),
    Point(Point3),
}

pub struct Sphere {
    location: Location,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(location: Location, radius: f64, material: Material) -> Object {
        Object::Sphere(Self {
            location,
            radius,
            material,
        })
    }

    #[inline(always)]
    fn center(&self, time: f64) -> Point3 {
        match self.location {
            Location::Ray(r) => r.at(time - r.time),
            Location::Point(p) => p,
        }
    }

    #[inline(always)]
    fn hit(&self, ray: Ray3, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let center = self.center(ray.time);

        let oc = ray.origin - center;
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
        let outward_normal = (point - center) / self.radius;
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

    pub fn hit(&self, ray: Ray3, t_min: f64, t_max: f64) -> Option<HitRecord> {
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
