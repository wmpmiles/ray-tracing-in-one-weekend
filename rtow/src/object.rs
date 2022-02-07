use crate::hit_record::HitRecord;
use crate::material::Material;
use geometry3d::*;
use std::cmp::Ordering;
use std::rc::Rc;

pub trait Object: CloneObject {
    fn hit(&self, ray: Ray3, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, t_min: f64, t_max: f64) -> Option<AABB>;
}

pub trait CloneObject {
    fn clone_object(&self) -> Box<dyn Object>;
}

impl<T> CloneObject for T
where
    T: Object + Clone + 'static,
{
    fn clone_object(&self) -> Box<dyn Object> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Object> {
    fn clone(&self) -> Self {
        self.clone_object()
    }
}

#[derive(Clone)]
pub struct Sphere {
    location: Ray3,
    radius: f64,
    material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(location: Ray3, radius: f64, material: Box<dyn Material>) -> Sphere {
        Sphere {
            location,
            radius,
            material,
        }
    }

    pub fn center(&self, time: f64) -> Point3 {
        self.location.at(time - self.location.time)
    }
}

impl Object for Sphere {
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
        let material = &*self.material;

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
    objects: Vec<Box<dyn Object>>,
}

impl List {
    pub fn new() -> List {
        List {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Object>) {
        self.objects.push(object);
    }
}

impl Object for List {
    fn hit(&self, ray: Ray3, t_min: f64, t_max: f64) -> Option<HitRecord> {
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

impl Default for List {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
pub struct BVHNode {
    aabb: AABB,
    left: Box<dyn Object>,
    right: Box<dyn Object>,
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

    fn object_lo(t0: f64, t1: f64) -> impl Fn(&Box<dyn Object>) -> Point3 {
        move |obj| obj.bounding_box(t0, t1).unwrap().lo()
    }

    fn axis(
        f: Rc<impl Fn(&Box<dyn Object>) -> Point3 + 'static>,
        g: impl Fn(Point3) -> f64 + 'static,
    ) -> impl Fn(&Box<dyn Object>) -> f64 {
        move |obj| g(f(obj))
    }

    fn cmp(
        f: impl Fn(&Box<dyn Object>) -> f64,
    ) -> impl Fn(&Box<dyn Object>, &Box<dyn Object>) -> Ordering {
        move |a, b| {
            let a = f(a);
            let b = f(b);
            a.partial_cmp(&b).unwrap()
        }
    }

    fn lr(
        objects: &mut [Box<dyn Object>],
        f: impl Fn(&Box<dyn Object>) -> f64,
        t0: f64,
        t1: f64,
    ) -> (Box<BVHNode>, Box<BVHNode>, f64) {
        objects.sort_unstable_by(Self::cmp(f));
        let (lhs, rhs) = objects.split_at_mut(objects.len() / 2);
        let left = Box::new(Self::from_vec(lhs, t0, t1));
        let right = Box::new(Self::from_vec(rhs, t0, t1));
        let vl = left.bounding_box(t0, t1).unwrap().volume();
        let vr = right.bounding_box(t0, t1).unwrap().volume();
        let ratio = vl.max(vr) / vl.min(vr);
        (left, right, ratio)
    }

    fn from_vec(objects: &mut [Box<dyn Object>], t0: f64, t1: f64) -> BVHNode {
        let lo = Rc::new(Self::object_lo(t0, t1));
        let x = Self::axis(lo.clone(), |p3| p3.x());
        let y = Self::axis(lo.clone(), |p3| p3.y());
        let z = Self::axis(lo, |p3| p3.z());

        let (left, right);
        if objects.len() <= 2 {
            left = objects[0].clone();
            right = if objects.len() == 2 {
                objects[1].clone()
            } else {
                left.clone()
            };
        } else {
            let (xl, xr, xratio) = Self::lr(objects, x, t0, t1);
            let (yl, yr, yratio) = Self::lr(objects, y, t0, t1);
            let (zl, zr, zratio) = Self::lr(objects, z, t0, t1);
            let smallest = xratio.min(yratio.min(zratio));
            if xratio == smallest {
                left = xl;
                right = xr;
            } else if yratio == smallest {
                left = yl;
                right = yr;
            } else {
                left = zl;
                right = zr;
            }
        }

        let left_aabb = left.bounding_box(t0, t1).unwrap();
        let right_aabb = right.bounding_box(t0, t1).unwrap();
        let aabb = AABB::merge(Some(left_aabb), Some(right_aabb)).unwrap();

        BVHNode { aabb, left, right }
    }
}

impl Object for BVHNode {
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        Some(self.aabb)
    }

    fn hit(&self, ray_in: Ray3, t_min: f64, t_max: f64) -> Option<HitRecord> {
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
