use crate::hit_record::HitRecord;
use crate::material::Material;
use geometry3d::*;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::rc::Rc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Object {
    Sphere(Sphere),
    Rect(Rect),
    List(List),
    BVHNode(BVHNode),
}

impl Object {
    pub fn hit(&mut self, ray: Ray3, t_range: TRange<f64>) -> Option<(HitRecord, &mut Material)> {
        match self {
            Object::Sphere(o) => o.hit(ray, t_range),
            Object::Rect(o) => o.hit(ray, t_range),
            Object::List(o) => o.hit(ray, t_range),
            Object::BVHNode(o) => o.hit(ray, t_range),
        }
    }

    pub fn bounding_box(&self, t_range: TRange<f64>) -> Option<AABB> {
        match self {
            Object::Sphere(o) => o.bounding_box(t_range),
            Object::Rect(o) => o.bounding_box(t_range),
            Object::List(o) => o.bounding_box(t_range),
            Object::BVHNode(o) => o.bounding_box(t_range),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sphere {
    location: Ray3,
    radius: f64,
    material: Material,
}

impl From<Sphere> for Object {
    fn from(s: Sphere) -> Object {
        Object::Sphere(s)
    }
}

impl Sphere {
    pub fn new(location: Ray3, radius: f64, material: Material) -> Sphere {
        Sphere {
            location,
            radius,
            material,
        }
    }

    pub fn center(&self, time: f64) -> Point3 {
        self.location.at(time - self.location.time)
    }

    /// Given a point (p) on a sphere of radius one, centered at the origin,
    /// calculates a uv mapping on the surface of the sphere such that u and v
    /// lie in [0, 1].
    pub fn uv(p: Point3) -> (f64, f64) {
        let pi = std::f64::consts::PI;

        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + pi;

        let u = phi / (2.0 * pi);
        let v = theta / pi;

        (u, v)
    }

    fn hit(&mut self, ray: Ray3, t_range: TRange<f64>) -> Option<(HitRecord, &mut Material)> {
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
        if !t_range.contains(&root) {
            root = (-half_b + sqrtd) / a;
            if !t_range.contains(&root) {
                return None;
            }
        }

        let t = root;
        let point = ray.at(t);
        let outward_normal = (point - center) / self.radius;
        let (u, v) = Sphere::uv(outward_normal.into());
        let material = &mut self.material;

        Some((
            HitRecord::new(point, outward_normal, ray, t, u, v),
            material,
        ))
    }

    fn bounding_box(&self, t_range: TRange<f64>) -> Option<AABB> {
        let rvec = Vec3::new(self.radius, self.radius, self.radius);

        let center0 = self.center(t_range.start);
        let center1 = self.center(t_range.end);

        let box0 = AABB::new(center0 - rvec, center0 + rvec);
        let box1 = AABB::new(center1 - rvec, center1 + rvec);

        AABB::merge(Some(box0), Some(box1))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct List {
    objects: Vec<Object>,
}

impl From<List> for Object {
    fn from(l: List) -> Object {
        Object::List(l)
    }
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

    fn hit(&mut self, ray: Ray3, t_range: TRange<f64>) -> Option<(HitRecord, &mut Material)> {
        let mut closest: Option<(HitRecord, &mut Material)> = None;

        for object in &mut self.objects {
            let t_max = match &closest {
                None => t_range.end,
                Some((rec, _mat)) => rec.t,
            };

            let new_range = TRange {
                start: t_range.start,
                end: t_max,
            };

            if let Some(hit) = object.hit(ray, new_range) {
                closest = Some(hit);
            }
        }

        closest
    }

    fn bounding_box(&self, t_range: TRange<f64>) -> Option<AABB> {
        let mut aabb = None;
        for object in &self.objects {
            aabb = AABB::merge(aabb, object.bounding_box(t_range));
        }
        aabb
    }
}

impl Default for List {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BVHNode {
    aabb: AABB,
    left: Box<Object>,
    right: Box<Object>,
}

impl From<BVHNode> for Object {
    fn from(n: BVHNode) -> Object {
        Object::BVHNode(n)
    }
}

impl BVHNode {
    /// Produces a Bounding Volume Hierarchy (BVH) from a `List` of `Object`.
    pub fn from_list(olist: &mut List, t_range: TRange<f64>) -> BVHNode {
        // preprocess list to remove all objects without AABBs
        let mut objects = &mut olist.objects[..];
        let mut i = 0;
        for j in 0..objects.len() {
            if objects[j].bounding_box(t_range).is_none() {
                objects.swap(i, j);
                i += 1;
            }
        }
        objects = &mut objects[i..];

        Self::from_vec(objects, t_range)
    }

    fn object_lo(t_range: TRange<f64>) -> impl Fn(&Object) -> Point3 {
        move |obj| obj.bounding_box(t_range).unwrap().lo()
    }

    fn axis(
        f: Rc<impl Fn(&Object) -> Point3 + 'static>,
        g: impl Fn(Point3) -> f64 + 'static,
    ) -> impl Fn(&Object) -> f64 {
        move |obj| g(f(obj))
    }

    fn cmp(f: impl Fn(&Object) -> f64) -> impl Fn(&Object, &Object) -> Ordering {
        move |a, b| {
            let a = f(a);
            let b = f(b);
            a.partial_cmp(&b).unwrap()
        }
    }

    fn lr(
        objects: &mut [Object],
        f: impl Fn(&Object) -> f64,
        t_range: TRange<f64>,
    ) -> (Object, Object, f64) {
        objects.sort_unstable_by(Self::cmp(f));
        let (lhs, rhs) = objects.split_at_mut(objects.len() / 2);
        let left = Object::from(Self::from_vec(lhs, t_range));
        let right = Object::from(Self::from_vec(rhs, t_range));
        let vl = left.bounding_box(t_range).unwrap().volume();
        let vr = right.bounding_box(t_range).unwrap().volume();
        let ratio = vl.max(vr) / vl.min(vr);
        (left, right, ratio)
    }

    fn from_vec(objects: &mut [Object], t_range: TRange<f64>) -> BVHNode {
        let lo = Rc::new(Self::object_lo(t_range));
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
            let (xl, xr, xratio) = Self::lr(objects, x, t_range);
            let (yl, yr, yratio) = Self::lr(objects, y, t_range);
            let (zl, zr, zratio) = Self::lr(objects, z, t_range);
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

        let left_aabb = left.bounding_box(t_range).unwrap();
        let right_aabb = right.bounding_box(t_range).unwrap();
        let aabb = AABB::merge(Some(left_aabb), Some(right_aabb)).unwrap();

        let left = Box::new(left);
        let right = Box::new(right);

        BVHNode { aabb, left, right }
    }

    fn bounding_box(&self, _t_range: TRange<f64>) -> Option<AABB> {
        Some(self.aabb)
    }

    fn hit(&mut self, ray_in: Ray3, t_range: TRange<f64>) -> Option<(HitRecord, &mut Material)> {
        if !self.aabb.hit(ray_in, t_range) {
            return None;
        }

        if let Some((hit_l, mat_l)) = self.left.hit(ray_in, t_range) {
            let new_range = TRange {
                start: t_range.start,
                end: hit_l.t,
            };
            if let Some((hit_r, mat_r)) = self.right.hit(ray_in, new_range) {
                Some((hit_r, mat_r))
            } else {
                Some((hit_l, mat_l))
            }
        } else {
            self.right.hit(ray_in, t_range)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rect {
    material: Material,
    a1: TRange<f64>,
    a2: TRange<f64>,
    a3: f64,
    axes: [Axis; 3],
}

impl Rect {
    fn permute(&self, ray_in: Ray3) -> Ray3 {
        let origin = ray_in.origin.permute(self.axes);
        let direction = ray_in.direction.permute(self.axes);
        let time = ray_in.time;
        Ray3 {
            origin,
            direction,
            time,
        }
    }

    fn hit(&mut self, ray_in: Ray3, t_range: TRange<f64>) -> Option<(HitRecord, &mut Material)> {
        let ray = self.permute(ray_in);

        let t = (self.a3 - ray.origin.z()) / ray.direction.z();
        if !t_range.contains(&t) {
            return None;
        }

        let p = ray.at(t);
        if !self.a1.contains(&p.x()) || !self.a2.contains(&p.y()) {
            return None;
        }

        let u = (p.x() - self.a1.start) / (self.a1.end - self.a1.start);
        let v = (p.y() - self.a2.start) / (self.a2.end - self.a2.start);

        let p = p.unpermute(self.axes);
        let outward_normal = Vec3::new(0.0, 0.0, -ray.direction.z().signum()).unpermute(self.axes);

        Some((
            HitRecord::new(p, outward_normal, ray_in, t, u, v),
            &mut self.material,
        ))
    }

    fn bounding_box(&self, _t_range: TRange<f64>) -> Option<AABB> {
        let epsilon = 1.0;
        let lower =
            Point3::new(self.a1.start, self.a2.start, self.a3 - epsilon).unpermute(self.axes);
        let upper = Point3::new(self.a1.end, self.a2.end, self.a3 + epsilon).unpermute(self.axes);
        Some(AABB::new(lower, upper))
    }
}
