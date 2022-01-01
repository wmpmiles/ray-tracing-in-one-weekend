use rand::distributions::{Distribution, Uniform};

#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct Vec3(pub f64, pub f64, pub f64);
pub type Point3 = Vec3;

impl Vec3 {
    pub const E0: Vec3 = Vec3 ( 1.0, 0.0, 0.0 );
    pub const E1: Vec3 = Vec3 ( 0.0, 1.0, 0.0 );
    pub const E2: Vec3 = Vec3 ( 0.0, 0.0, 1.0 );

    pub const fn new() -> Vec3 {
        Vec3(0.0, 0.0, 0.0)
    }
    pub fn from(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3(e0, e1, e2)
    }

    pub const fn from_const(a: f64, b: f64, c: f64) -> Vec3 {
        Vec3(a, b, c)
    }

    pub fn scalar(s: f64) -> Vec3 {
        Vec3(s, s, s)
    }

    pub fn random() -> Vec3 {
        Vec3(rand::random(), rand::random(), rand::random())
    }

    pub fn random_range(range: std::ops::RangeInclusive<f64>) -> Vec3 {
        let between = Uniform::from(range);
        let mut rng = rand::thread_rng();

        Vec3(
            between.sample(&mut rng),
            between.sample(&mut rng),
            between.sample(&mut rng),
        )
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let point = Point3::random_range(-1.0..=1.0);
            if point.quadrature() < 1.0 {
                return point;
            }
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        Vec3::random_in_unit_sphere().unit_vector()
    }

    pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
        let in_unit_sphere = Vec3::random_in_unit_sphere();
        if in_unit_sphere.dot(normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    pub fn length(self) -> f64 {
        self.quadrature().sqrt()
    }

    pub fn quadrature(self) -> f64 {
        self.dot(self)
    }

    pub fn unit_vector(self) -> Vec3 {
        self.scalar_div(self.length())
    }

    pub fn scalar_mul(self, scalar: f64) -> Vec3 {
        self * Vec3::scalar(scalar)
    }

    pub fn scalar_div(self, scalar: f64) -> Vec3 {
        self / Vec3::scalar(scalar)
    }

    pub fn dot(self, other: Vec3) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    pub fn cross(self, rhs: Vec3) -> Vec3 {
        Vec3(
            self.1 * rhs.2 - self.2 * rhs.1,
            self.2 * rhs.0 - self.0 * rhs.2,
            self.0 * rhs.1 - self.1 * rhs.0,
        )
    }

    pub fn elementwise<T>(self, transform: T) -> Vec3
    where
        T: Fn(f64) -> f64,
    {
        Vec3(transform(self.0), transform(self.1), transform(self.2))
    }

    pub fn near_zero(self) -> bool {
        self.quadrature() == 0.0
    }

    pub fn reflect(self, normal: Vec3) -> Vec3 {
        self - normal.scalar_mul(2.0 * self.dot(normal))
    }

    pub fn refract(self, normal: Vec3, eta_over_eta_prime: f64) -> Vec3 {
        let cos_theta = (-normal).dot(self);
        let r_in_perp = self + normal.scalar_mul(cos_theta);
        let r_out_perp = r_in_perp.scalar_mul(eta_over_eta_prime);
        let parallel_len = (1.0 - r_out_perp.quadrature()).sqrt();
        let r_out_parallel = -(normal.scalar_mul(parallel_len));
        r_out_perp + r_out_parallel
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl std::ops::Div for Vec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Vec3(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2)
    }
}

impl std::ops::DivAssign for Vec3 {
    fn div_assign(&mut self, other: Self) {
        *self = Self(self.0 / other.0, self.1 / other.1, self.2 / other.2)
    }
}

impl std::ops::Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl std::ops::MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Self) {
        *self = Self(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl std::ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const UNIT_RANGE: std::ops::RangeInclusive<f64> = 0.9999999999..=1.0000000001;

    fn nearly_equal(a: f64, b: f64) -> bool {
        UNIT_RANGE.contains(&(a - b + 1.0))
    }

    #[test]
    fn new() {
        let new = Vec3::new();
        assert_eq!(new.0, 0.0);
        assert_eq!(new.1, 0.0);
        assert_eq!(new.2, 0.0);
    }

    #[test]
    fn from() {
        let from = Vec3::from(0.3, 0.5, 0.8);
        assert_eq!(from.0, 0.3);
        assert_eq!(from.1, 0.5);
        assert_eq!(from.2, 0.8);
    }

    #[test]
    fn from_const() {
        let from_const = Vec3::from_const(0.3, 0.5, 0.8);
        assert_eq!(from_const.0, 0.3);
        assert_eq!(from_const.1, 0.5);
        assert_eq!(from_const.2, 0.8);
    }

    #[test]
    fn scalar() {
        let vec = Vec3::scalar(0.777);
        assert_eq!(vec.0, 0.777);
        assert_eq!(vec.1, 0.777);
        assert_eq!(vec.2, 0.777);
    }

    fn test_range(range: std::ops::RangeInclusive<f64>, vec: Vec3) {
        assert!(range.contains(&vec.0));
        assert!(range.contains(&vec.1));
        assert!(range.contains(&vec.2));
    }

    #[test]
    fn random() {
        let mut previous = Vec3::new();
        for _ in 0..100 {
            let vec = Vec3::random();
            test_range(0.0..=1.0, vec);
            assert_ne!(previous, vec);
            previous = vec;
        }
    }

    #[test]
    fn random_range() {
        let range1 = 10.0..=100.0;
        let range2 = -100.0..=100.0;
        let range3 = -10.0..=-9.0;
        for _ in 0..100 {
            test_range(range1.clone(), Vec3::random_range(range1.clone()));
            test_range(range2.clone(), Vec3::random_range(range2.clone()));
            test_range(range3.clone(), Vec3::random_range(range3.clone()));
        }
    }

    #[test]
    fn random_in_unit_sphere() {
        let mut previous = Vec3::new();
        for _ in 0..100 {
            let vec = Vec3::random_in_unit_sphere();
            assert!(vec.quadrature() <= 1.0);
            assert_ne!(previous, vec);
            previous = vec;
        }
    }

    #[test]
    fn random_unit_vector() {
        let mut previous = Vec3::new();
        for _ in 0..100 {
            let vec = Vec3::random_unit_vector();
            assert!(UNIT_RANGE.contains(&vec.length()));
            assert_ne!(previous, vec);
            previous = vec;
        }
    }

    #[test]
    fn random_in_hemisphere() {
        let mut previous = Vec3::new();
        for _ in 0..100 {
            let normal = Vec3::random_unit_vector();
            let vec = Vec3::random_in_hemisphere(normal);
            assert!(vec.length() <= 1.0);
            assert!(vec.dot(normal) >= 0.0);
            assert_ne!(previous, vec);
            previous = vec;
        }
    }

    #[test]
    fn length() {
        assert_eq!(Vec3::new().length(), 0.0);

        assert_eq!(Vec3::from_const(0.0, 0.0, 1.0).length(), 1.0);
        assert_eq!(Vec3::from_const(0.0, 1.0, 0.0).length(), 1.0);
        assert_eq!(Vec3::from_const(1.0, 0.0, 0.0).length(), 1.0);

        assert_eq!(Vec3::from_const(0.0, 3.0, 4.0).length(), 5.0);
        assert_eq!(Vec3::from_const(3.0, 0.0, 4.0).length(), 5.0);
        assert_eq!(Vec3::from_const(3.0, 4.0, 0.0).length(), 5.0);
        assert_eq!(Vec3::from_const(0.0, 5.0, 12.0).length(), 13.0);
        assert_eq!(Vec3::from_const(5.0, 0.0, 12.0).length(), 13.0);
        assert_eq!(Vec3::from_const(5.0, 12.0, 0.0).length(), 13.0);

        assert_eq!(Vec3::from_const(1.0, 2.0, 2.0).length(), 3.0);
        assert_eq!(Vec3::from_const(2.0, 3.0, 6.0).length(), 7.0);
        assert_eq!(Vec3::from_const(4.0, 4.0, 7.0).length(), 9.0);
        assert_eq!(Vec3::from_const(1.0, 4.0, 8.0).length(), 9.0);
    }

    #[test]
    fn quadrature() {
        assert_eq!(Vec3::new().quadrature(), 0.0);

        assert_eq!(Vec3::from_const(0.0, 0.0, 1.0).quadrature(), 1.0);
        assert_eq!(Vec3::from_const(0.0, 1.0, 0.0).quadrature(), 1.0);
        assert_eq!(Vec3::from_const(1.0, 0.0, 0.0).quadrature(), 1.0);

        assert_eq!(Vec3::from_const(0.0, 3.0, 4.0).quadrature(), 25.0);
        assert_eq!(Vec3::from_const(3.0, 0.0, 4.0).quadrature(), 25.0);
        assert_eq!(Vec3::from_const(3.0, 4.0, 0.0).quadrature(), 25.0);
        assert_eq!(Vec3::from_const(0.0, 5.0, 12.0).quadrature(), 169.0);
        assert_eq!(Vec3::from_const(5.0, 0.0, 12.0).quadrature(), 169.0);
        assert_eq!(Vec3::from_const(5.0, 12.0, 0.0).quadrature(), 169.0);

        assert_eq!(Vec3::from_const(1.0, 2.0, 2.0).quadrature(), 9.0);
        assert_eq!(Vec3::from_const(2.0, 3.0, 6.0).quadrature(), 49.0);
        assert_eq!(Vec3::from_const(4.0, 4.0, 7.0).quadrature(), 81.0);
        assert_eq!(Vec3::from_const(1.0, 4.0, 8.0).quadrature(), 81.0);
    }

    #[test]
    fn unit_vector() {
        for _ in 0..100 {
            let vec = Vec3::random().unit_vector();
            assert!(UNIT_RANGE.contains(&vec.length()));
        }
    }

    #[test]
    fn scalar_mul() {
        for _ in 0..100 {
            let vec = Vec3::random();
            let scalar: f64 = rand::random();
            let mul = vec.scalar_mul(scalar);
            assert!(nearly_equal(vec.length() * scalar.abs(), mul.length()));
        }
    }

    #[test]
    fn scalar_div() {
        for _ in 0..100 {
            let vec = Vec3::random();
            let scalar: f64 = rand::random();
            let div = vec.scalar_div(scalar);
            assert!(nearly_equal(vec.length() / scalar.abs(), div.length()));
        }
    }

    #[test]
    fn dot() {
        for _ in 0..100 {
            let vec = Vec3::random();
            assert_eq!(vec.dot(Vec3::new()), 0.0);
            assert_eq!(vec.dot(Vec3::E0), vec.0);
            assert_eq!(vec.dot(Vec3::E1), vec.1);
            assert_eq!(vec.dot(Vec3::E2), vec.2);
            assert_eq!(vec.dot(Vec3::scalar(1.0)), vec.0 + vec.1 + vec.2);
            assert_eq!(
                vec.dot(Vec3::from_const(1.0, 2.0, 3.0)),
                1.0 * vec.0 + 2.0 * vec.1 + 3.0 * vec.2
            );
        }
    }

    #[test]
    fn cross() {
        assert_eq!(Vec3::E0.cross(Vec3::E1), Vec3::E2);
        assert_eq!(Vec3::E1.cross(Vec3::E2), Vec3::E0);
        assert_eq!(Vec3::E0.cross(Vec3::E1), Vec3::E2);
    }

    #[test]
    fn elementwise() {}

    #[test]
    fn near_zero() {}

    #[test]
    fn reflect() {}

    #[test]
    fn refract() {}
}
