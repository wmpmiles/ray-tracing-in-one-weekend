//! # Geometry 3D
//!
//! `geometry3d` is a collection of structures and associated functions that
//! reperesent 3D Cartesian geometry concepts such as points, vectors, and rays
//! using double precision floating-point values.
//!
//! ## Caution
//!
//! It should be noted that the values and methods here are all floating-point
//! approximations and as such will potentially have small errors and will
//! display numeric instability in some cases.

use ntuple::*;
use ntuple_derive::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Axis {
    X = 0,
    Y = 1,
    Z = 2,
}

pub trait Permute {
    fn permute(self, axes: [Axis; 3]) -> Self;
}

impl<T> Permute for T
where
    T: NTupleNewtype<f64, 3>
{
    fn permute(self, axes: [Axis; 3]) -> T {
        let perms = axes.map(|x| x as usize);
        T::from(self.ntuple().permute(perms))
    }
}

/// 3D Euclidean vector.
///
/// # Examples
/// ```
/// use geometry3d::Vec3;
///
/// let v1 = Vec3::new(1.0, 2.0, 3.0);
/// let v2 = Vec3::e1();
///
/// assert_eq!(v1.dot(v2), 2.0);
/// ```
///
/// `e0()` through `e2()` are the standard basis vectors.
///
#[derive(Debug, PartialEq, Copy, Clone, Default, Serialize, Deserialize, NTupleNewtype)]
pub struct Vec3(NTuple<f64, 3>);

impl Vec3 {
    /// The first standard basis vector (1, 0, 0).
    pub fn e0() -> Self {
        Vec3(ntuple!(1.0, 0.0, 0.0))
    }

    /// The second standard basis vector (0, 1, 0).
    pub fn e1() -> Self {
        Vec3(ntuple!(0.0, 1.0, 0.0))
    }

    /// The third standard basis vector (0, 0, 1).
    pub fn e2() -> Self {
        Vec3(ntuple!(0.0, 0.0, 1.0))
    }

    /// Create a new `Vec3`. The vector (x, y, z) is equivalent to
    /// `x * e0 + y * e1 + z * e2`.
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(ntuple!(x, y, z))
    }

    /// The coefficient of the first basis vector.
    pub fn x(self) -> f64 {
        self.0[0]
    }

    /// The coefficient of the second basis vector.
    pub fn y(self) -> f64 {
        self.0[1]
    }

    /// The coefficient of the third basis vector.
    pub fn z(self) -> f64 {
        self.0[2]
    }

    /// The square of the Euclidean length of the vector.
    pub fn quadrance(self) -> f64 {
        let s = Vec3(self.0.map(|x| x * x));
        s.x() + s.y() + s.z()
    }

    /// The Euclidean length of the vector.
    pub fn length(self) -> f64 {
        self.quadrance().sqrt()
    }

    /// The vector scaled such that the length of the resulting vector is 1.
    pub fn unit(self) -> Option<Self> {
        let length = self.length();
        if length == 0.0 {
            None
        } else {
            Some(Vec3(self.0.map(|x| x / length)))
        }
    }

    /// The dot product of two vectors.
    pub fn dot(self, rhs: Vec3) -> f64 {
        self.0.combine(rhs.0, |x, y| x * y).reduce(|acc, x| acc + x)
    }

    /// The cross product of the left hand vector by the right hand vector i.e.
    /// `a.cross(b)` is the cross of `a` by `b`.
    pub fn cross(self, rhs: Vec3) -> Vec3 {
        Self(ntuple!(
            self.y() * rhs.z() - self.z() * rhs.y(),
            self.z() * rhs.x() - self.x() * rhs.z(),
            self.x() * rhs.y() - self.y() * rhs.x()
        ))
    }

    /// The projection of the left vector onto the right vector i.e.
    /// `a.projection(b)` is the projection of `a` onto `b`.
    pub fn projection(self, b: Self) -> Self {
        let a = self;
        a.dot(b) / b.quadrance() * b
    }

    /// The reflection of the left vector across the plane through the origin
    /// and normal to the right vector.
    pub fn reflection(self, normal: Self) -> Self {
        self - 2.0 * self.projection(normal)
    }
}

/// Sum of two vectors.
impl std::ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Vec3 {
        Self(self.0.combine(rhs.0, |x, y| x + y))
    }
}

/// Difference of two vectors.
impl std::ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Vec3 {
        Self(self.0.combine(rhs.0, |x, y| x - y))
    }
}

/// Negative of the vector.
impl std::ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Self(self.0.map(|x| -x))
    }
}

/// Scale the vector by the scalar.
impl std::ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3(rhs.0.map(|x| x * self))
    }
}

/// Scale vector by the reciprocal of the divisor.
impl std::ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Vec3 {
        Self(self.0.map(|x| x / rhs))
    }
}

/// Position vector from point.
impl std::convert::From<Point3> for Vec3 {
    fn from(point: Point3) -> Vec3 {
        Vec3(ntuple!(point.x(), point.y(), point.z()))
    }
}

/// 3D Cartesian point.
///
/// # Examples
/// ```
/// use geometry3d::{Point3, Vec3};
/// let p1 = Point3::new(1.0, 2.0, 3.0);
/// let p2 = Point3::default();
/// let difference = Vec3::new(1.0, 2.0, 3.0);
///
/// assert_eq!(p1 - p2, difference);
/// ```
#[derive(Debug, PartialEq, Copy, Clone, Default, Serialize, Deserialize, NTupleNewtype)]
pub struct Point3(NTuple<f64, 3>);

impl Point3 {
    /// Create a new `Point3` with cartesian coordinates (`x`, `y`, `z`).
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(ntuple!(x, y, z))
    }

    /// The x coordinate.
    pub fn x(self) -> f64 {
        self.0[0]
    }

    /// The y coordinate.
    pub fn y(self) -> f64 {
        self.0[1]
    }

    /// The z coordinate.
    pub fn z(self) -> f64 {
        self.0[2]
    }
}

/// Convert from position vector to point.
impl std::convert::From<Vec3> for Point3 {
    fn from(vec: Vec3) -> Self {
        Point3(vec.0)
    }
}

/// Point from the starting point displaced by the vector.
impl std::ops::Add<Vec3> for Point3 {
    type Output = Self;

    fn add(self, rhs: Vec3) -> Self::Output {
        Self(self.0.combine(rhs.0, |x, y| x + y))
    }
}

/// Point from the starting point displaced by the negative of the vector.
impl std::ops::Sub<Vec3> for Point3 {
    type Output = Self;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Self(self.0.combine(rhs.0, |x, y| x - y))
    }
}

/// The vector from the right-hand point to the left-hand point.
impl std::ops::Sub<Point3> for Point3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3(self.0.combine(rhs.0, |x, y| x - y))
    }
}

#[derive(Debug, PartialEq, Copy, Clone, Default, Serialize, Deserialize)]
pub struct Ray3 {
    pub origin: Point3,
    pub direction: Vec3,
    pub time: f64,
}

impl Ray3 {
    pub fn at(self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct AABB {
    lo: Point3,
    hi: Point3,
}

impl AABB {
    fn pmin(a: Point3, b: Point3) -> Point3 {
        Point3(a.0.combine(b.0, |x, y| x.min(y)))
    }

    fn pmax(a: Point3, b: Point3) -> Point3 {
        Point3(a.0.combine(b.0, |x, y| x.max(y)))
    }

    pub fn new(a: Point3, b: Point3) -> AABB {
        assert!(
            a.0.combine(b.0, | x, y | x != y).reduce(|acc, x| acc && x),
            "AABB extents must have a non-zero distance in all 3 dimensions."
        );
        let lo = Self::pmin(a, b);
        let hi = Self::pmax(a, b);
        AABB { lo, hi }
    }

    pub fn lo(self) -> Point3 {
        self.lo
    }

    pub fn hi(self) -> Point3 {
        self.hi
    }

    /// Calculates the per axis t-values for the ray passing through the planes
    /// that intersect at the given point.
    fn t(extent: Point3, ray: Ray3) -> NTuple<f64, 3> {
        (extent - ray.origin)
            .0
            .combine(ray.direction.0, |x, y| x / y)
    }

    /// Determine if a ray intersects with the axis-aligned bounding box for
    /// t-values between `t_min` and `t_max`.
    ///
    /// Note that `t_min` must be strictly less than `t_max`.
    pub fn hit(self, ray: Ray3, t_range: TRange<f64>) -> bool {
        assert!(
            t_range.start < t_range.end,
            "t_min must be less than t_max for aabb hit calculation."
        );

        // Calculate t-values for the ray intersections with the planes
        // described by the extents.
        //
        // 0 values in the direction vector still work as origins that lie
        // between the planes will produce + and - infinities which will be
        // dropped when compared with t_max/t_min, but outside of/on the
        // boundary of will produce a situtaion where both the lower and upper
        // values are + or - infinity which will always lead to t_min >= t_max.
        let t0 = Self::t(self.lo(), ray);
        let t1 = Self::t(self.hi(), ray);

        // intersection t values sorted
        let t_lower = t0.combine(t1, |x, y| x.min(y));
        let t_upper = t0.combine(t1, |x, y| x.max(y));

        // determine largest lower t-value, and smallest upper t-value
        let t_min = t_lower.fold(t_range.start, |x, y| x.max(y));
        let t_max = t_upper.fold(t_range.end, |x, y| x.min(y));

        // if the ray passes through the volume of the AABB (not just the edge)
        t_min < t_max
    }

    /// Takes two optioned AABBs and merges them. If both are Some then a true
    /// merge is performed. If only one is Some then the Some is returned. If
    /// both are None then None is returned.
    pub fn merge(a: Option<AABB>, b: Option<AABB>) -> Option<AABB> {
        if let Some(a) = a {
            if let Some(b) = b {
                let lo = Self::pmin(a.lo, b.lo);
                let hi = Self::pmax(a.hi, b.hi);
                Some(AABB { lo, hi })
            } else {
                Some(a)
            }
        } else {
            b
        }
    }

    pub fn volume(self) -> f64 {
        (self.hi - self.lo).0.reduce(|acc, x| acc * x).abs()
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct TRange<T>
{
    pub start: T,
    pub end: T,
}

impl<T> TRange<T> 
where
    T: PartialOrd
{
    pub fn new(start: T, end: T) -> TRange<T> {
        TRange { start, end }
    }

    pub fn contains(&self, i: &T) -> bool {
        self.start.le(i) && self.end.ge(i)
    }
}

