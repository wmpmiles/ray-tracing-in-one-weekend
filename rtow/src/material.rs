use crate::hit_record::HitRecord;
use crate::color::FloatRgb;
use crate::random::Random;
use geometry3d::*;

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

impl Material {
    pub fn scatter(self, ray_in: Ray3, rec: &HitRecord) -> Option<(FloatRgb, Ray3)> {
        match self {
            Self::Lambertian(lambertian) => lambertian.scatter(rec),
            Self::Metal(metal) => metal.scatter(rec, ray_in),
            Self::Dielectric(dielectric) => dielectric.scatter(rec, ray_in),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Lambertian {
    albedo: FloatRgb,
}

impl Lambertian {
    pub fn new(albedo: FloatRgb) -> Material {
        Material::Lambertian( Lambertian { albedo } )
    }

    fn scatter(self, rec: &HitRecord) -> Option<(FloatRgb, Ray3)> {
        // reject internal reflections from opaque material
        if !rec.front_face {
            return None;
        }

        // unit normal + unit vector guaranteed to lie in or above the
        // tangent plane, thus only need to account for the case of
        // a direction vector of zero length
        let mut rng = rand::thread_rng();
        let mut rng = Random::new(&mut rng);
        let scatter = rec.normal + rng.unit_vector();
        let direction = match scatter.unit() {
            Some(vec) => vec,
            None => rec.normal,
        };

        let origin = rec.point;

        Some((self.albedo, Ray3 { origin, direction }))
    }
}

#[derive(Copy, Clone)]
pub struct Metal {
    albedo: FloatRgb,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: FloatRgb, fuzz: f64) -> Material {
        Material::Metal( Metal { albedo, fuzz } )
    }

    fn scatter(&self, rec: &HitRecord, ray_in: Ray3) -> Option<(FloatRgb, Ray3)> {
        // reject internal reflections from opaque materials
        if !rec.front_face {
            return None;
        }

        // calculate pure specular reflection vector
        let reflection = ray_in.direction.reflection(rec.normal);
        let mut rng = rand::thread_rng();
        let mut rng = Random::new(&mut rng);
        let mut direction;
        loop {
            direction = reflection + self.fuzz * rng.in_unit_sphere();

            // only accept direction vectors that have some length and
            // lie above the plane tangent to the sphere at the point
            // of reflection
            if direction.quadrance() == 0.0 || direction.dot(rec.normal) <= 0.0 {
                continue;
            } else {
                break;
            }
        }
        direction = direction.unit().unwrap();

        let origin = rec.point;

        Some((self.albedo, Ray3 { origin, direction }))
    }
}

#[derive(Copy, Clone)]
pub struct Dielectric {
    index_of_refraction: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Material {
        Material::Dielectric( Dielectric { index_of_refraction } )
    }

    fn reflectance(cosine: f64, refractive_index: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let mut r0 = (1.0 - refractive_index) / (1.0 + refractive_index);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }

    fn refraction(vec: Vec3, normal: Vec3, eta_over_eta_prime: f64) -> Vec3 {
        let cos_theta = -vec.dot(normal);
        let r_in_perp = vec + cos_theta * normal;
        let r_out_perp = eta_over_eta_prime * r_in_perp;
        let parallel_len = (1.0 - r_out_perp.quadrance()).sqrt();
        let r_out_parallel = -parallel_len * normal;
        r_out_perp + r_out_parallel
    }

    fn scatter(&self, rec: &HitRecord, ray_in: Ray3) -> Option<(FloatRgb, Ray3)> {
        // calculate refraction ratio depending on in internal/external reflection
        let refraction_ratio = match rec.front_face {
            true => 1.0 / self.index_of_refraction,
            false => self.index_of_refraction,
        };

        let cos_theta = -rec.normal.dot(ray_in.direction);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let reflectance = Self::reflectance(cos_theta, self.index_of_refraction);
        let reflect = cannot_refract || reflectance > rand::random();

        let direction = match reflect {
            true => ray_in.direction.reflection(rec.normal),
            false => Self::refraction(ray_in.direction, rec.normal, refraction_ratio),
        };

        let origin = rec.point;

        Some((FloatRgb::new(1.0, 1.0, 1.0), Ray3 { origin, direction }))
    }
}
