use geometry3d::*;
use rand::rngs::StdRng;
use rand::SeedableRng;
use rtow::camera::*;
use rtow::color::*;
use rtow::config::*;
use rtow::image::Image;
use rtow::material::*;
use rtow::object::*;
use rtow::random::Random;
use rtow::sampler::SquareSampler;
use rtow::texture::*;

fn main() -> std::io::Result<()> {
    // Image
    let image_config = ImageConfig {
        filename: String::from("render.png"),
        width: 400,
        height: 300,
    };
    let mut image = Image::new(image_config);

    // World
    let mut world = random_scene();

    // Camera
    let camera_config = CameraConfig {
        look_from: Point3::new(13.0, 2.0, 3.0),
        look_at: Point3::new(0.0, 0.0, 0.0),
        up: Vec3::new(0.0, 1.0, 0.0),
        vertical_fov: 20.0,
        focus_distance: 10.0,
        aspect_ratio: image.aspect_ratio,
        aperture: 0.1,
        time_min: 0.0,
        time_max: 1.0,
    };

    let camera = Camera::new(camera_config);

    let opt_world = Object::BVHNode(BVHNode::from_list(&mut world, camera.time_min, camera.time_max));

    // Sampler
    let sampler_config = SamplerConfig {
        n: 3,
        max_depth: 50,
    };
    let sampler = SquareSampler::new(sampler_config, &image);

    // using bottom left as (0,0)
    for (x, y) in image.iter() {
        let mut pixel_color = FRgbAccumulator::new();

        for (u, v) in sampler.iter(x, y) {
            let ray = camera.get_ray(u, v);
            pixel_color += ray_color(ray, &opt_world, sampler.max_depth);
        }

        image.add_pixel(pixel_color.average().into());
    }

    image.write()?;
    eprint!("\nDone.\n");

    Ok(())
}

fn ray_color(ray: Ray3, world: &Object, depth: u32) -> FloatRgb {
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
    let ground_texture = Texture::CheckerTexture(CheckerTexture::new(
        FloatRgb::new(0.2, 0.3, 0.1).into(),
        FloatRgb::new(0.9, 0.9, 0.9).into(),
    ));
    let ground_material = Material::Lambertian(Lambertian::new(ground_texture));
    let ground = Object::Sphere(Sphere::new(ground_center, ground_radius, ground_material));
    world.add(ground);

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
                let material: Material = if choose_mat < 0.8 {
                    center.direction = perturbed;
                    let albedo = rng.color() * rng.color();
                    Material::Lambertian(Lambertian::new(albedo.into()))
                } else if choose_mat < 0.95 {
                    let albedo = rng.color().mix(white, 0.5);
                    let fuzz = rng.random::<f64>() / 2.0;
                    Material::Metal(Metal::new(albedo, fuzz))
                } else {
                    Material::Dielectric(Dielectric::new(1.5))
                };

                let sphere = Object::Sphere(Sphere::new(center, SMALL_RADIUS, material));
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

    let material1 = Material::Dielectric(Dielectric::new(1.5));
    let material2 = Material::Lambertian(Lambertian::new(FloatRgb::new(0.4, 0.2, 0.1).into()));
    let material3 = Material::Metal(Metal::new(FloatRgb::new(0.7, 0.6, 0.5), 0.0));

    let sphere1 = Object::Sphere(Sphere::new(center1, LARGE_RADIUS, material1));
    let sphere2 = Object::Sphere(Sphere::new(center2, LARGE_RADIUS, material2));
    let sphere3 = Object::Sphere(Sphere::new(center3, LARGE_RADIUS, material3));

    world.add(sphere1);
    world.add(sphere2);
    world.add(sphere3);

    world
}
