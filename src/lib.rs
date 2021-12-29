pub mod vec3 {
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
}

pub mod color {
    use crate::vec3::Vec3;
    pub type Color = Vec3;

    fn map_byte(val: f64) -> u8 {
        let val = val.clamp(0.0, 1.0);
        const RATIO: f64 = 255.999;
        (RATIO * val) as u8
    }

    impl Color {
        pub fn write(self, samples_per_pixel: u32, data: &mut Vec<u8>) {
            let scale = 1.0 / samples_per_pixel as f64;
            let color = self.elementwise(|x| (scale * x).sqrt());

            data.push(map_byte(color.0));
            data.push(map_byte(color.1));
            data.push(map_byte(color.2));
            data.push(255);
        }
    }
}

pub mod ray {
    use crate::vec3::Point3;
    use crate::vec3::Vec3;

    #[derive(Default)]
    pub struct Ray {
        pub origin: Point3,
        pub direction: Vec3,
    }

    impl Ray {
        pub fn at(&self, t: f64) -> Point3 {
            self.origin + self.direction.scalar_mul(t)
        }
    }
}

pub mod hit_record {
    use crate::material::*;
    use crate::ray::Ray;
    use crate::vec3::*;
    use std::rc::Rc;

    pub struct HitRecord {
        pub point: Point3,
        pub normal: Vec3,
        pub material: Rc<dyn Material>,
        pub t: f64,
        pub front_face: bool,
    }

    impl HitRecord {
        pub fn new() -> HitRecord {
            HitRecord {
                material: Rc::new(Lambertian {
                    albedo: Default::default(),
                }),
                point: Default::default(),
                normal: Default::default(),
                t: Default::default(),
                front_face: Default::default(),
            }
        }

        pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
            self.front_face = ray.direction.dot(outward_normal) < 0.0;
            self.normal = if self.front_face {
                outward_normal
            } else {
                -outward_normal
            };
        }
    }

    impl Default for HitRecord {
        fn default() -> Self {
            Self::new()
        }
    }
}

pub mod object {
    use crate::hit_record::HitRecord;
    use crate::material::Material;
    use crate::ray::Ray;
    use crate::vec3::*;
    use std::rc::Rc;

    pub trait Object {
        fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
    }

    pub struct Sphere {
        centre: Point3,
        radius: f64,
        material: Rc<dyn Material>,
    }

    impl Sphere {
        pub fn from(centre: Point3, radius: f64, material: Rc<dyn Material>) -> Sphere {
            Sphere {
                centre,
                radius,
                material,
            }
        }
    }

    impl Object for Sphere {
        fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
            let oc = ray.origin - self.centre;
            let a = ray.direction.dot(ray.direction);
            let half_b = ray.direction.dot(oc);
            let c = oc.dot(oc) - self.radius * self.radius;
            let delta = half_b * half_b - a * c;
            if delta < 0.0 {
                // no solutions -> no intersection
                return false;
            }

            // find the nearest root that lies in the acceptable range
            let sqrtd = delta.sqrt();
            let mut root = (-half_b - sqrtd) / a;
            if root < t_min || t_max < root {
                root = (-half_b + sqrtd) / a;
                if root < t_min || t_max < root {
                    return false;
                }
            }

            rec.t = root;
            rec.point = ray.at(rec.t);
            let outward_normal = (rec.point - self.centre).scalar_div(self.radius);
            rec.set_face_normal(ray, outward_normal);
            rec.material = Rc::clone(&self.material);

            true
        }
    }

    pub struct ObjectList {
        objects: Vec<Rc<dyn Object>>,
    }

    impl ObjectList {
        pub fn new() -> ObjectList {
            ObjectList {
                objects: Vec::new(),
            }
        }

        pub fn add(&mut self, object: Rc<dyn Object>) {
            self.objects.push(object);
        }
    }

    impl Object for ObjectList {
        fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
            let mut hit_anything = false;
            let mut closest_so_far = t_max;

            for object in &self.objects {
                if object.hit(ray, t_min, closest_so_far, rec) {
                    hit_anything = true;
                    closest_so_far = rec.t;
                }
            }

            hit_anything
        }
    }

    impl Default for ObjectList {
        fn default() -> Self {
            Self::new()
        }
    }
}

pub mod camera {
    use crate::ray::Ray;
    use crate::vec3::*;

    pub struct Camera {
        origin: Point3,
        horizontal: Vec3,
        vertical: Vec3,
        lower_left_corner: Point3,
    }

    impl Camera {
        pub const ASPECT_RATIO: f64 = 16.0 / 9.0;
        const VIEWPORT_HEIGHT: f64 = 2.0;
        const VIEWPORT_WIDTH: f64 = Self::ASPECT_RATIO * Self::VIEWPORT_HEIGHT;
        const FOCAL_LENGTH: f64 = 1.0;

        pub fn new() -> Camera {
            let origin = Point3::new();
            let horizontal = Vec3::from(Self::VIEWPORT_WIDTH, 0.0, 0.0);
            let vertical = Vec3::from(0.0, Self::VIEWPORT_HEIGHT, 0.0);
            let lower_left_corner = origin
                - horizontal.scalar_div(2.0)
                - vertical.scalar_div(2.0)
                - Vec3::from(0.0, 0.0, Self::FOCAL_LENGTH);

            Camera {
                origin,
                horizontal,
                vertical,
                lower_left_corner,
            }
        }
        pub fn get_ray(&self, u: f64, v: f64) -> Ray {
            Ray {
                origin: self.origin,
                direction: self.lower_left_corner
                    + self.horizontal.scalar_mul(u)
                    + self.vertical.scalar_mul(v)
                    - self.origin,
            }
        }
    }

    impl Default for Camera {
        fn default() -> Self {
            Self::new()
        }
    }
}

pub mod material {
    use crate::color::Color;
    use crate::hit_record::HitRecord;
    use crate::ray::Ray;
    use crate::vec3::*;

    pub trait Material {
        fn scatter(
            &self,
            ray_in: &Ray,
            rec: &HitRecord,
            attenuation: &mut Color,
            scattered: &mut Ray,
        ) -> bool;
    }

    pub struct Lambertian {
        pub albedo: Color,
    }

    impl Lambertian {
        pub fn new(albedo: Color) -> Lambertian {
            Lambertian { albedo }
        }
    }

    impl Material for Lambertian {
        fn scatter(
            &self,
            _ray_in: &Ray,
            rec: &HitRecord,
            attenuation: &mut Color,
            scattered: &mut Ray,
        ) -> bool {
            if !rec.front_face {
                return false;
            }

            let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

            if scatter_direction.near_zero() {
                scatter_direction = rec.normal;
            }

            *scattered = Ray {
                origin: rec.point,
                direction: scatter_direction,
            };
            *attenuation = self.albedo;
            true
        }
    }

    pub struct Metal {
        pub albedo: Color,
        pub fuzz: f64,
    }

    impl Metal {
        pub fn new(albedo: Color, fuzz: f64) -> Metal {
            let fuzz = fuzz.clamp(0.0, 1.0);
            Metal { albedo, fuzz }
        }
    }

    impl Material for Metal {
        fn scatter(
            &self,
            ray_in: &Ray,
            rec: &HitRecord,
            attenuation: &mut Color,
            scattered: &mut Ray,
        ) -> bool {
            if !rec.front_face {
                return false;
            }

            let origin = rec.point;

            let reflection = ray_in.direction.unit_vector().reflect(rec.normal);
            let mut direction;

            loop {
                direction = reflection + Vec3::random_in_unit_sphere().scalar_mul(self.fuzz);

                if direction.near_zero() || direction.dot(rec.normal) <= 0.0 {
                    continue;
                } else {
                    break;
                }
            }

            *scattered = Ray { origin, direction };
            *attenuation = self.albedo;
            true
        }
    }

    pub struct Dielectric {
        index_of_refraction: f64,
    }

    impl Dielectric {
        pub fn new(index_of_refraction: f64) -> Self {
            Dielectric {
                index_of_refraction,
            }
        }

        fn reflectance(cosine: f64, refractive_index: f64) -> f64 {
            // Use Schlick's approximation for reflectance.
            let mut r0 = (1.0 - refractive_index) / (1.0 + refractive_index);
            r0 = r0 * r0;
            r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
        }
    }

    impl Material for Dielectric {
        fn scatter(
            &self,
            ray_in: &Ray,
            rec: &HitRecord,
            attenuation: &mut Color,
            scattered: &mut Ray,
        ) -> bool {
            *attenuation = Color::from_const(1.0, 1.0, 1.0);
            let origin = rec.point;

            let refraction_ratio = match rec.front_face {
                true => 1.0 / self.index_of_refraction,
                false => self.index_of_refraction,
            };

            let unit_direction = ray_in.direction.unit_vector();
            let cos_theta = rec.normal.dot(-unit_direction).clamp(-1.0, 1.0);
            let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

            let cannot_refract = refraction_ratio * sin_theta > 1.0;
            let reflect = cannot_refract || 
                Dielectric::reflectance(cos_theta, refraction_ratio) > rand::random();

            let direction = match reflect {
                true => unit_direction.reflect(rec.normal),
                false => unit_direction.refract(rec.normal, refraction_ratio),
            };

            *scattered = Ray { origin, direction };
            true
        }
    }
}
