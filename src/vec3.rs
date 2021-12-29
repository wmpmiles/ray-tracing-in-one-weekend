use rand::distributions::{Distribution, Uniform};

#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct Vec3(pub f64, pub f64, pub f64);
pub type Point3 = Vec3;

impl Vec3 {
    pub const fn new() -> Vec3 {
        Vec3(0_f64, 0_f64, 0_f64)
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
