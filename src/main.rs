use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;
use rtow::camera::Camera;
use rtow::image::Image;
use rtow::material::*;
use rtow::object::*;
use rtow::ray::Ray;
use rtow::vec3::*;
use rtow::sampler::SquareSampler;
use rtow::config::Config;
use std::env;

fn main() -> std::io::Result<()> {
    // Args
    let config = Config::parse(env::args());

    // Image
    let mut image = Image::new(config.aspect_ratio, config.image_width);

    // World
    let world = random_scene();

    // Camera
    let lookfrom = Point3::from_const(13.0, 2.0, 3.0);
    let lookat = Point3::from_const(0.0, 0.0, 0.0);
    let vup = Vec3::from_const(0.0, 1.0, 0.0);
    let vfov = 20.0;
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        image.aspect_ratio,
        aperture,
        dist_to_focus,
    );
    
    // Sampler
    let sampler = SquareSampler::new(image.width, image.height, config.sampler_n);

    // using bottom left as (0,0)
    for (x, y) in image.iter() {
        let mut pixel_color = Color::new();

        for (u, v) in sampler.iter(x, y) {
            let ray = camera.get_ray(u, v);
            pixel_color += ray_color(&ray, &world, config.max_depth);
        }

        image.add_pixel(pixel_color, sampler.samples());
    }

    image.write(&config.filename)?;
    eprint!("\nDone.\n");

    Ok(())
}

fn ray_color(ray: &Ray, world: &Object, depth: u32) -> Color {
    const ONE: Color = Color::from_const(1.0, 1.0, 1.0);
    const BASE: Color = Color::from_const(0.5, 0.7, 1.0);
    const NONE: Color = Color::from_const(0.0, 0.0, 0.0);
    const MIN: f64 = 0.001; // minimize hitting the same point due to floating point approximation

    if depth == 0 {
        NONE
    } else if let Some(rec) = world.hit(ray, MIN, f64::INFINITY) {
        if let Some((attenuation, ray)) = rec.material.scatter(ray, &rec) {
            attenuation * ray_color(&ray, world, depth - 1)
        } else {
            NONE
        }
    } else {
        let t = 0.5 * (ray.direction.1 + 1.0);
        ONE.scalar_mul(1.0 - t) + BASE.scalar_mul(t)
    }
}

fn random_scene() -> Object {
    const SMALL_RADIUS: f64 = 0.2;

    let mut rng = StdRng::seed_from_u64(0);
    let random: &mut dyn FnMut() -> f64 = &mut || rng.gen();

    let mut world = List::new();

    let ground_center = Point3::from_const(0.0, -1000.0, 0.0);
    let ground_radius = 1000.0;
    let ground_material = Lambertian::new(Color::from_const(0.5, 0.5, 0.5));
    let ground = Sphere::new(ground_center, ground_radius, ground_material);
    world.add(ground);

    for a in -11..11 {
        for b in -11..11 {
            let a = a as f64;
            let b = b as f64;

            let choose_mat: f64 = random();
            let center = Point3::from(a + 0.9 * random(), 0.2, b + 0.9 * random());

            if (center - Point3::from_const(4.0, 0.2, 0.0)).length() > 0.9 {
                let material = if choose_mat < 0.8 {
                    let albedo = Color::from(random(), random(), random())
                        * Color::from(random(), random(), random());
                    Lambertian::new(albedo)
                } else if choose_mat < 0.95 {
                    let albedo = Color::from(random(), random(), random()).scalar_mul(0.5)
                        + Color::scalar(0.5);
                    let fuzz = random() / 2.0;
                    Metal::new(albedo, fuzz)
                } else {
                    Dielectric::new(1.5)
                };

                let sphere = Sphere::new(center, SMALL_RADIUS, material);
                world.add(sphere);
            }
        }
    }

    let center1 = Point3::from_const(0.0, 1.0, 0.0);
    let center2 = Point3::from_const(-4.0, 1.0, 0.0);
    let center3 = Point3::from_const(4.0, 1.0, 0.0);

    const LARGE_RADIUS: f64 = 1.0;

    let material1 = Dielectric::new(1.5);
    let material2 = Lambertian::new(Color::from_const(0.4, 0.2, 0.1));
    let material3 = Metal::new(Color::from_const(0.7, 0.6, 0.5), 0.0);

    let sphere1 = Sphere::new(center1, LARGE_RADIUS, material1);
    let sphere2 = Sphere::new(center2, LARGE_RADIUS, material2);
    let sphere3 = Sphere::new(center3, LARGE_RADIUS, material3);

    world.add(sphere1);
    world.add(sphere2);
    world.add(sphere3);

    Object::List(world)
}
