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
        // reject internal reflections from opaque material
        if !rec.front_face {
            return false;
        }

        // unit normal + unit vector guaranteed to lie in or above the
        // tangent plane, thus only need to account for the case of
        // a direction vector of zero length
        let scatter = rec.normal + Vec3::random_unit_vector();
        let direction = match scatter.unit_vector() {
            Some(vec) => vec,
            None => rec.normal,
        };

        let origin = rec.point;

        *scattered = Ray { origin, direction };
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
        // reject internal reflections from opaque materials
        if !rec.front_face {
            return false;
        }

        // calculate pure specular reflection vector
        let reflection = ray_in.direction.reflect(rec.normal);
        let mut direction;
        loop {
            direction = reflection + Vec3::random_in_unit_sphere().scalar_mul(self.fuzz);

            // only accept direction vectors that have some length and
            // lie above the plane tangent to the sphere at the point
            // of reflection
            if direction.quadrance() == 0.0 || direction.dot(rec.normal) <= 0.0 {
                continue;
            } else {
                break;
            }
        }
        direction = direction.unit_vector().unwrap();

        let origin = rec.point;

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
        // calculate refraction ratio depending on in internal/external reflection
        let refraction_ratio = match rec.front_face {
            true => 1.0 / self.index_of_refraction,
            false => self.index_of_refraction,
        };

        let cos_theta = -rec.normal.dot(ray_in.direction);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let reflectance = Dielectric::reflectance(cos_theta, self.index_of_refraction);
        let reflect = cannot_refract || reflectance > rand::random();

        let direction = match reflect {
            true => ray_in.direction.reflect(rec.normal),
            false => ray_in.direction.refract(rec.normal, refraction_ratio),
        };

        let origin = rec.point;

        *scattered = Ray { origin, direction };
        *attenuation = Color::from_const(1.0, 1.0, 1.0);
        true
    }
}
