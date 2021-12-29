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
        let reflect =
            cannot_refract || Dielectric::reflectance(cos_theta, refraction_ratio) > rand::random();

        let direction = match reflect {
            true => unit_direction.reflect(rec.normal),
            false => unit_direction.refract(rec.normal, refraction_ratio),
        };

        *scattered = Ray { origin, direction };
        true
    }
}
