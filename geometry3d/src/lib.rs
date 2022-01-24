use n_tuple::*;

#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct Vec3(NTuple<f64, 3>);

/* Behaviours:
 * - cannonical basis vectors
 * - create vec3 from x, y, z
 * - element access
 * - calc quadrance
 * - calc length
 * - to unit vector
 * - dot product
 * - cross product
 * - add
 * - subtract
 * - negate
 * - scalar multiplication
 * - scalar division
 * - projection
 * - reflection
 * - position vector from point
 */

impl Vec3 {
    pub fn e0() -> Self {
        Vec3(ntuple!(1.0, 0.0, 0.0))
    }

    pub fn e1() -> Self {
        Vec3(ntuple!(0.0, 1.0, 0.0))
    }

    pub fn e2() -> Self {
        Vec3(ntuple!(0.0, 0.0, 1.0))
    }

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(ntuple!(x, y, z))
    }

    pub fn x(self) -> f64 {
        self.0[0]
    }

    pub fn y(self) -> f64 {
        self.0[1]
    }

    pub fn z(self) -> f64 {
        self.0[2]
    }

    pub fn quadrance(self) -> f64 {
        let s = Vec3(self.0.map(|x| x * x));
        s.x() + s.y() + s.z()
    }

    pub fn length(self) -> f64 {
        self.quadrance().sqrt()
    }

    pub fn unit(self) -> Option<Self> {
        let length = self.length();
        if length == 0.0 {
            None
        } else {
            Some(Vec3(self.0.map(|x| x / length)))
        }
    }

    #[inline(always)]
    pub fn dot(self, rhs: Vec3) -> f64 {
        self.0.combine(rhs.0, |x, y| x * y).reduce(|acc, x| acc + x)
    }

    pub fn cross(self, rhs: Vec3) -> Vec3 {
        Self(ntuple!(
            self.y() * rhs.z() - self.z() * rhs.y(),
            self.z() * rhs.x() - self.x() * rhs.z(),
            self.x() * rhs.y() - self.y() * rhs.x()
        ))
    }

    pub fn projection(self, b: Self) -> Self {
        let a = self;
        a.dot(b) / b.quadrance() * b
    }

    pub fn reflection(self, normal: Self) -> Self {
        self - 2.0 * self.projection(normal)
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0.combine(rhs.0, |x, y| x + y))
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0.combine(rhs.0, |x, y| x - y))
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(self.0.map(|x| -x))
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0.map(|x| x * rhs))
    }
}

impl std::ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(rhs.0.map(|x| x * self))
    }
}

impl std::ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self(self.0.map(|x| x / rhs))
    }
}

impl std::convert::From<Point3> for Vec3 {
    fn from(point: Point3) -> Self {
        Vec3(ntuple!(point.x(), point.y(), point.z()))
    }
}

#[cfg(test)]
mod vec3_tests {
    use super::*;

    #[test]
    fn cannonical_basis_vectors() {
        assert_eq!(Vec3::e0().x(), 1.0);
        assert_eq!(Vec3::e0().y(), 0.0);
        assert_eq!(Vec3::e0().z(), 0.0);

        assert_eq!(Vec3::e1().x(), 0.0);
        assert_eq!(Vec3::e1().y(), 1.0);
        assert_eq!(Vec3::e1().z(), 0.0);

        assert_eq!(Vec3::e2().x(), 0.0);
        assert_eq!(Vec3::e2().y(), 0.0);
        assert_eq!(Vec3::e2().z(), 1.0);
    }

    #[test]
    fn new_vector() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v.x(), 1.0);
        assert_eq!(v.y(), 2.0);
        assert_eq!(v.z(), 3.0);
    }

    #[test]
    fn vector_quadrance() {
        let v = Vec3::new(1.0, 2.0, 2.0);
        assert_eq!(v.quadrance(), 9.0);
    }

    #[test]
    fn vector_length() {
        let v = Vec3::new(2.0, 3.0, 6.0);
        assert_eq!(v.length(), 7.0);
    }

    #[test]
    fn unit_vector() {
        let v = Vec3::new(1.0, 2.0, 2.0).unit().unwrap();
        assert_eq!(v.x(), 1.0 / 3.0);
        assert_eq!(v.y(), 2.0 / 3.0);
        assert_eq!(v.z(), 2.0 / 3.0);
        let n = Vec3::new(0.0, 0.0, 0.0).unit();
        assert_eq!(n, None);
    }

    #[test]
    fn dot_product() {
        let v1 = Vec3::new(0.0, 1.0, 2.0);
        let v2 = Vec3::new(2.0, 1.0, 0.0);
        assert_eq!(v1.dot(v2), 1.0);
    }

    #[test]
    fn cross_product() {
        assert_eq!(Vec3::e0().cross(Vec3::e1()), Vec3::e2());
    }

    #[test]
    fn add_vectors() {
        let v1 = Vec3::e0() + Vec3::e1() + Vec3::e2();
        let v2 = Vec3::new(1.0, 1.0, 1.0);
        assert_eq!(v1, v2);
    }

    #[test]
    fn subtract_vectors() {
        let v1 = -Vec3::e0() - Vec3::e1() - Vec3::e2();
        let v2 = Vec3::new(-1.0, -1.0, -1.0);
        assert_eq!(v1, v2);
    }

    #[test]
    fn negate_vector() {
        let v1 = Vec3::new(1.0, 1.0, 1.0);
        let v2 = Vec3::new(-1.0, -1.0, -1.0);
        assert_eq!(-v1, v2);
    }

    #[test]
    fn scalar_multiplication() {
        let v1 = Vec3::new(1.0, 1.0, 1.0);
        let v2 = Vec3::new(3.0, 3.0, 3.0);
        assert_eq!(v1 * 3.0, v2);
        assert_eq!(3.0 * v1, v2);
    }

    #[test]
    fn scalar_division() {
        let v1 = Vec3::new(5.0, 5.0, 5.0);
        let v2 = Vec3::new(2.5, 2.5, 2.5);
        assert_eq!(v1 / 2.0, v2);
    }

    #[test]
    fn vector_projection() {
        let v1 = Vec3::e0() + Vec3::e1();
        let v2 = 2.0 * Vec3::e0();
        assert_eq!(v1.projection(v2), Vec3::e0());
    }

    #[test]
    fn position_vector() {
        let p = Point3::new(0.1, 0.2, 0.3);
        let v: Vec3 = p.into();
        assert_eq!(v.x(), 0.1);
        assert_eq!(v.y(), 0.2);
        assert_eq!(v.z(), 0.3);
    }
}

#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct Point3(NTuple<f64, 3>);

/* Behaviours:
 * - create from x, y, z
 * - from position vector
 * - add vector
 * - sub vector
 * - difference of 2 ponits (A - B = vector from B to A)
 */

impl Point3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(ntuple!(x, y, z))
    }
    pub fn x(self) -> f64 {
        self.0[0]
    }

    pub fn y(self) -> f64 {
        self.0[1]
    }

    pub fn z(self) -> f64 {
        self.0[2]
    }
}

impl std::convert::From<Vec3> for Point3 {
    fn from(vec: Vec3) -> Self {
        Point3(vec.0)
    }
}

impl std::ops::Add<Vec3> for Point3 {
    type Output = Self;

    fn add(self, rhs: Vec3) -> Self::Output {
        Self(self.0.combine(rhs.0, |x, y| x + y))
    }
}

impl std::ops::Sub<Vec3> for Point3 {
    type Output = Self;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Self(self.0.combine(rhs.0, |x, y| x - y))
    }
}

impl std::ops::Sub<Point3> for Point3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3(self.0.combine(rhs.0, |x, y| x - y))
    }
}

#[cfg(test)]
mod point3_tests {
    use super::*;

    #[test]
    fn new_point() {
        let p = Point3::new(22.0, 33.0, 44.0);
        assert_eq!(p.x(), 22.0);
        assert_eq!(p.y(), 33.0);
        assert_eq!(p.z(), 44.0);
    }

    #[test]
    fn from_vector() {
        let v = Vec3::new(-5.0, -8.0, -13.0);
        let p: Point3 = v.into();
        assert_eq!(p.x(), -5.0);
        assert_eq!(p.y(), -8.0);
        assert_eq!(p.z(), -13.0);
    }

    #[test]
    fn add_vector_to_point() {
        let p1 = Point3::new(0.0, 0.0, 0.0);
        let v = Vec3::new(0.1, 0.4, 0.9);
        let p2 = p1 + v;
        assert_eq!(p2, v.into());
    }

    #[test]
    fn sub_vector_from_point() {
        let p1 = Point3::new(0.0, 0.0, 0.0);
        let v = Vec3::new(0.1, 0.4, 0.9);
        let p2 = p1 - v;
        assert_eq!(p2, (-v).into());
    }

    #[test]
    fn difference_of_two_ponits() {
        let p1 = Point3::new(0.0, 0.0, 0.0);
        let p2 = Point3::new(4.0, 8.0, 16.0);
        assert_eq!(p2 - p1, Vec3::from(p2));
    }
}

#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct Ray3 {
    pub origin: Point3,
    pub direction: Vec3,
    pub time: f64,
}

/* Behaviours:
 * - Create ray at time
 * - Access ray origin, direction, and time
 * - Calculate point on ray at some multiple of direction
 */

impl Ray3 {
    pub fn at(self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}

#[cfg(test)]
mod ray3_tests {
    use super::*;

    #[test]
    fn create_and_access() {
        let origin = Point3::new(0.0, 0.0, 0.0);
        let direction = Vec3::e0();
        let time = 0.0;
        let ray = Ray3 { origin, direction, time };
        assert_eq!(origin, ray.origin);
        assert_eq!(direction, ray.direction);
        assert_eq!(time, ray.time);
    }

    #[test]
    fn at() {
        let origin = Point3::new(0.0, 0.0, 0.0);
        let direction = Vec3::e0();
        let time = 0.0;
        let ray = Ray3 { origin, direction, time };

        let p1 = origin + direction;
        let p2 = origin + 2.0 * direction;
        assert_eq!(ray.at(1.0), p1);
        assert_eq!(ray.at(2.0), p2);
    }
}
