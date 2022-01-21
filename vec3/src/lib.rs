#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/* Behaviours:
 * - cannonical basis vectors
 * - create vec3 from x, y, z
 * - elementwise
 * - combine
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
 */

impl Vec3 {
    pub const E0: Vec3 = Vec3 { x: 1.0, y: 0.0, z: 0.0 };
    pub const E1: Vec3 = Vec3 { x: 0.0, y: 1.0, z: 0.0 };
    pub const E2: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 1.0 };

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn elementwise<F>(self, f: F) -> Self
    where
        F: Fn(f64) -> f64,
    {
        Self {
            x: f(self.x),
            y: f(self.y),
            z: f(self.z),
        }
    }

    pub fn combine<F>(self, rhs: Vec3, f: F) -> Self
    where
        F: Fn(f64, f64) -> f64,
    {
        Self {
            x: f(self.x, rhs.x),
            y: f(self.y, rhs.y),
            z: f(self.z, rhs.z),
        }
    }

    pub fn quadrance(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(self) -> f64 {
        self.quadrance().sqrt()
    }

    pub fn unit(self) -> Vec3 {
        let length = self.length();
        self.elementwise(|x| x / length)
    }

    pub fn dot(self, rhs: Vec3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(self, rhs: Vec3) -> Vec3 {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
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
        self.combine(rhs, |x, y| x + y)
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.combine(rhs, |x, y| x - y)
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.elementwise(|x| -x)
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        self.elementwise(|x| x * rhs)
    }
}

impl std::ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs.elementwise(|x| x * self)
    }
}

impl std::ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self.elementwise(|x| x / rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cannonical_basis_vectors() {
        assert_eq!(Vec3::E0.x, 1.0);
        assert_eq!(Vec3::E0.y, 0.0);
        assert_eq!(Vec3::E0.z, 0.0);

        assert_eq!(Vec3::E1.x, 0.0);
        assert_eq!(Vec3::E1.y, 1.0);
        assert_eq!(Vec3::E1.z, 0.0);

        assert_eq!(Vec3::E2.x, 0.0);
        assert_eq!(Vec3::E2.y, 0.0);
        assert_eq!(Vec3::E2.z, 1.0);
    }

    #[test]
    fn new_vector() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn elementwise_operation() {
        let v = Vec3::new(2.0, 3.0, 4.0).elementwise(|x| x * x);
        assert_eq!(v.x, 4.0);
        assert_eq!(v.y, 9.0);
        assert_eq!(v.z, 16.0);
    }

    #[test]
    fn combine_vectors() {
        let v1 = Vec3::new(3.0, 4.0, 5.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let v3 = v1.combine(v2, |x, y| x * y);
        assert_eq!(v3.x, 12.0);
        assert_eq!(v3.y, 20.0);
        assert_eq!(v3.z, 30.0);
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
        let v = Vec3::new(1.0, 2.0, 2.0).unit();
        assert_eq!(v.x, 1.0 / 3.0);
        assert_eq!(v.y, 2.0 / 3.0);
        assert_eq!(v.z, 2.0 / 3.0);
    }

    #[test]
    fn dot_product() {
        let v1 = Vec3::new(0.0, 1.0, 2.0);
        let v2 = Vec3::new(2.0, 1.0, 0.0);
        assert_eq!(v1.dot(v2), 1.0);
    }

    #[test]
    fn cross_product() {
        assert_eq!(Vec3::E0.cross(Vec3::E1), Vec3::E2);
    }

    #[test]
    fn add_vectors() {
        let v1 = Vec3::E0 + Vec3::E1 + Vec3::E2;
        let v2 = Vec3::new(1.0, 1.0, 1.0);
        assert_eq!(v1, v2);
    }

    #[test]
    fn subtract_vectors() {
        let v1 = -Vec3::E0 - Vec3::E1 - Vec3::E2;
        let v2 = Vec3::new(-1.0, -1.0, -1.0);
        assert_eq!(v1, v2);
    }

    #[test]
    fn negate_vector() {
        let v1 = Vec3::new(1.0, 1.0, 1.0);
        assert_eq!(-v1, v1.elementwise(|x| -x));
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
        let v1 = Vec3::E0 + Vec3::E1;
        let v2 = 2.0 * Vec3::E0;
        assert_eq!(v1.projection(v2), Vec3::E0);
    }
}
