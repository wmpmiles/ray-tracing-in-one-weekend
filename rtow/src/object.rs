use crate::hit_record::HitRecord;
use crate::material::Material;
use geometry3d::*;

#[derive(Clone)]
pub enum Object {
    Sphere(Sphere),
    List(List),
    BVHNode(BVHNode),
}

impl Object {
    pub fn hit(&self, ray: Ray3, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self {
            Object::Sphere(sphere) => sphere.hit(ray, t_min, t_max),
            Object::List(list) => list.hit(ray, t_min, t_max),
            Object::BVHNode(node) => node.hit(ray, t_min, t_max),
        }
    }

    pub fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        match self {
            Object::Sphere(sphere) => sphere.bounding_box(time0, time1),
            Object::List(list) => list.bounding_box(time0, time1),
            Object::BVHNode(node) => node.bounding_box(time0, time1),
        }
    }
}

#[derive(Clone)]
pub struct Sphere {
    location: Ray3,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(location: Ray3, radius: f64, material: Material) -> Sphere {
        Sphere {
            location,
            radius,
            material,
        }
    }

    fn center(&self, time: f64) -> Point3 {
        self.location.at(time - self.location.time)
    }

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

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        let rvec = Vec3::new(self.radius, self.radius, self.radius);

        let center0 = self.center(time0);
        let center1 = self.center(time1);

        let box0 = AABB::new(center0 - rvec, center0 + rvec);
        let box1 = AABB::new(center1 - rvec, center1 + rvec);

        AABB::merge(Some(box0), Some(box1))
    }
}

#[derive(Clone)]
pub struct List {
    objects: Vec<Object>,
}

impl List {
    pub fn new() -> List {
        List {
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

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        let mut aabb = None;
        for object in &self.objects {
            aabb = AABB::merge(aabb, object.bounding_box(time0, time1));
        }
        aabb
    }
}

#[derive(Clone)]
pub struct BVHNode {
    aabb: AABB,
    left: Box<Object>,
    right: Box<Object>,
}

impl BVHNode {
    /// Produces a Bounding Volume Hierarchy (BVH) from a `List` of `Object`.
    pub fn from_list(olist: &mut List, time0: f64, time1: f64) -> BVHNode {
        // preprocess list to remove all objects without AABBs
        let mut objects = &mut olist.objects[..];
        let mut i = 0;
        for j in 0..objects.len() {
            if objects[j].bounding_box(time0, time1).is_none() {
                objects.swap(i, j);
                i += 1;
            }
        }
        objects = &mut objects[i..];

        Self::from_vec(objects, time0, time1)
    }

    fn object_lo(obj: &Object, time0: f64, time1: f64) -> Point3 {
        obj.bounding_box(time0, time1).unwrap().lo()
    }

    fn from_vec(objects: &mut [Object], time0: f64, time1: f64) -> BVHNode {
        objects.sort_unstable_by(|a, b| {
            let a = Self::object_lo(a, time0, time1).x();
            let b = Self::object_lo(b, time0, time1).x();
            a.partial_cmp(&b).unwrap()
        });

        let (left, right);
        if objects.len() <= 2 {
            left = objects[0].clone();
            right = if objects.len() == 2 {
                objects[1].clone()
            } else {
                left.clone()
            };
        } else {
            let (lhs, rhs) = objects.split_at_mut(objects.len() / 2);
            left = Object::BVHNode(Self::from_vec(lhs, time0, time1));
            right = Object::BVHNode(Self::from_vec(rhs, time0, time1));
        }

        let left_aabb = left.bounding_box(time0, time1).unwrap();
        let right_aabb = right.bounding_box(time0, time1).unwrap();
        let aabb = AABB::merge(Some(left_aabb), Some(right_aabb)).unwrap();

        let left = Box::new(left);
        let right = Box::new(right);

        BVHNode {
            aabb,
            left,
            right,
        }
    }

    pub fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        Some(self.aabb)
    }

    pub fn hit(&self, ray_in: Ray3, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.aabb.hit(ray_in, t_min, t_max) {
            return None;
        }

        if let Some(hit_l) = self.left.hit(ray_in, t_min, t_max) {
            if let Some(hit_r) = self.right.hit(ray_in, t_min, hit_l.t) {
                Some(hit_r)
            } else {
                Some(hit_l)
            }
        } else {
            self.right.hit(ray_in, t_min, t_max)
        }
    }
}
