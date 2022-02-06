use geometry3d::*;
use rand::rngs::StdRng;
use rand::SeedableRng;
use rtow::camera::Camera;
use rtow::color::*;
use rtow::config::Config;
use rtow::image::Image;
use rtow::material::*;
use rtow::object::*;
use rtow::random::Random;
use rtow::sampler::SquareSampler;
use std::env;

fn main() -> std::io::Result<()> {
    // Args
    let config = Config::parse(env::args());

    // Image
    let mut image = Image::new(config.aspect_ratio, config.image_width);

    // World
    let mut world = random_scene();

    // Camera
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let vfov = 20.0;
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let time_min = Some(0.0);
    let time_max = Some(1.0);

    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        image.aspect_ratio,
        aperture,
        dist_to_focus,
        time_min,
        time_max,
    );

    let opt_world = BVHNode::from_list(&mut world, time_min.unwrap(), time_max.unwrap());

    // Sampler
    let sampler = SquareSampler::new(image.width, image.height, config.sampler_n);

    // using bottom left as (0,0)
    for (x, y) in image.iter() {
        let mut pixel_color = FRgbAccumulator::new();

        for (u, v) in sampler.iter(x, y) {
            let ray = camera.get_ray(u, v);
            pixel_color += ray_color(ray, &opt_world, config.max_depth);
        }

        image.add_pixel(pixel_color.average().into());
    }

    image.write(&config.filename)?;
    eprint!("\nDone.\n");

    Ok(())
}

fn ray_color(ray: Ray3, world: &dyn Object, depth: u32) -> FloatRgb {
    let one = FloatRgb::new(1.0, 1.0, 1.0);
    let base = FloatRgb::new(0.5, 0.7, 1.0);
    let none = FloatRgb::new(0.0, 0.0, 0.0);
    const MIN: f64 = 0.001; // minimize hitting the same point due to floating point approximation

    if depth == 0 {
        none
    } else if let Some(rec) = world.hit(ray, MIN, f64::INFINITY) {
        if let Some((attenuation, ray)) = rec.material.scatter(ray, &rec) {
            attenuation * ray_color(ray, world, depth - 1)
        } else {
            none
        }
    } else {
        let t = 0.5 * (ray.direction.y() + 1.0);
        base.mix(one, t)
    }
}

fn random_scene() -> List {
    const SMALL_RADIUS: f64 = 0.2;
    const TIME: f64 = 0.0;
    let white = FloatRgb::new(1.0, 1.0, 1.0);
    let perturbed = Vec3::new(0.0, 0.25, 0.0);
    let still = Vec3::new(0.0, 0.0, 0.0);

    let mut rng = Random::new(StdRng::seed_from_u64(0));

    let mut world = List::new();

    let ground_center = Ray3 {
        origin: Point3::new(0.0, -1000.0, 0.0),
        direction: still,
        time: TIME,
    };
    let ground_radius = 1000.0;
    let ground_material = Lambertian::new(FloatRgb::new(0.5, 0.5, 0.5));
    let ground = Sphere::new(ground_center, ground_radius, ground_material);
    world.add(Box::new(ground));

    for a in -11..11 {
        for b in -11..11 {
            let a = a as f64;
            let b = b as f64;

            let choose_mat: f64 = rng.random();
            let center_point = Point3::new(
                a + 0.9 * rng.random::<f64>(),
                0.2,
                b + 0.9 * rng.random::<f64>(),
            );
            let mut center = Ray3 {
                origin: center_point,
                direction: still,
                time: TIME,
            };

            if (center_point - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let material = if choose_mat < 0.8 {
                    center.direction = perturbed;
                    let albedo = rng.color() * rng.color();
                    Lambertian::new(albedo)
                } else if choose_mat < 0.95 {
                    let albedo = rng.color().mix(white, 0.5);
                    let fuzz = rng.random::<f64>() / 2.0;
                    Metal::new(albedo, fuzz)
                } else {
                    Dielectric::new(1.5)
                };

                let sphere = Box::new(Sphere::new(center, SMALL_RADIUS, material));
                world.add(sphere);
            }
        }
    }

    let center1 = Ray3 {
        origin: Point3::new(0.0, 1.0, 0.0),
        direction: still,
        time: TIME,
    };
    let center2 = Ray3 {
        origin: Point3::new(-4.0, 1.0, 0.0),
        direction: still,
        time: TIME,
    };
    let center3 = Ray3 {
        origin: Point3::new(4.0, 1.0, 0.0),
        direction: still,
        time: TIME,
    };

    const LARGE_RADIUS: f64 = 1.0;

    let material1 = Dielectric::new(1.5);
    let material2 = Lambertian::new(FloatRgb::new(0.4, 0.2, 0.1));
    let material3 = Metal::new(FloatRgb::new(0.7, 0.6, 0.5), 0.0);

    let sphere1 = Box::new(Sphere::new(center1, LARGE_RADIUS, material1));
    let sphere2 = Box::new(Sphere::new(center2, LARGE_RADIUS, material2));
    let sphere3 = Box::new(Sphere::new(center3, LARGE_RADIUS, material3));

    world.add(sphere1);
    world.add(sphere2);
    world.add(sphere3);

    world
}
