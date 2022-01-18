use rtow::camera::Camera;
use rtow::material::*;
use rtow::object::*;
use rtow::ray::Ray;
use rtow::vec3::*;
use rtow::image::Image;
use std::rc::Rc;
use rand::rngs::StdRng;
use rand::SeedableRng;
use rand::Rng;

fn main() -> std::io::Result<()> {
    // Image
    let mut image = Image::new(3.0 / 2.0, 400);

    // World
    let world = random_scene();

    // Camera
    let lookfrom = Point3::from_const(13.0, 2.0, 3.0);
    let lookat = Point3::from_const(0.0, 0.0, 0.0);
    let vup = Vec3::from_const(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        image.aspect_ratio,
        aperture,
        dist_to_focus,
    );

    // Render
    const SAMPLES_PER_PIXEL: u32 = 10;
    const MAX_DEPTH: u32 = 50;

    for j in (0..image.height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..image.width {
            let mut pixel_color = Color::new();

            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (rand::random::<f64>() + i as f64) / (image.width - 1) as f64;
                let v = (rand::random::<f64>() + j as f64) / (image.height - 1) as f64;

                let ray = camera.get_ray(u, v);

                pixel_color += ray_color(&ray, &world, MAX_DEPTH);
            }

            image.add_pixel(pixel_color, SAMPLES_PER_PIXEL);
        }
    }

    image.write(r"render.png")?;
    eprint!("\nDone.\n");

    Ok(())
}

fn ray_color(ray: &Ray, world: &dyn Object, depth: u32) -> Color {
    const ONE: Color = Color::from_const(1.0, 1.0, 1.0);
    const BASE: Color = Color::from_const(0.5, 0.7, 1.0);
    const NONE: Color = Color::from_const(0.0, 0.0, 0.0);
    const MIN: f64 = 0.001; // minimize hitting the same point due to floating point approximation

    if depth == 0 {
        return NONE;
    }

    let mut rec = Default::default();

    if world.hit(ray, MIN, f64::INFINITY, &mut rec) {
        let mut scattered: Ray = Default::default();
        let mut attenuation: Color = Default::default();

        if rec
            .material
            .scatter(ray, &rec, &mut attenuation, &mut scattered)
        {
            attenuation * ray_color(&scattered, world, depth - 1)
        } else {
            NONE
        }
    } else {
        let t = 0.5 * (ray.direction.1 + 1.0);

        ONE.scalar_mul(1.0 - t) + BASE.scalar_mul(t)
    }
}

fn random_scene() -> ObjectList {
    let mut rng = StdRng::seed_from_u64(0);
    let random: &mut dyn FnMut() -> f64 = &mut || rng.gen();

    let mut world = ObjectList::new();

    let ground_material = Rc::new(Lambertian {
        albedo: Color::from_const(0.5, 0.5, 0.5),
    });
    world.add(Rc::new(
        Sphere::from(
            Point3::from_const(0.0, -1000.0, 0.0),
            1000.0,
            ground_material,
        )
        .unwrap(),
    ));

    for a in -11..11 {
        for b in -11..11 {
            let a = a as f64;
            let b = b as f64;

            let choose_mat: f64 = random();
            let center = Point3::from(
                a + 0.9 * random(),
                0.2,
                b + 0.9 * random(),
            );

            if (center - Point3::from_const(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::from(random(), random(), random()) * Color::from(random(), random(), random());
                    let material = Lambertian { albedo };
                    let sphere = Sphere::from(center, 0.2, Rc::new(material)).unwrap();
                    world.add(Rc::new(sphere));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::from(random(), random(), random()).scalar_mul(0.5) + Color::scalar(0.5);
                    let fuzz = random() / 2.0;
                    let material = Metal { albedo, fuzz };
                    let sphere = Sphere::from(center, 0.2, Rc::new(material)).unwrap();
                    world.add(Rc::new(sphere));
                } else {
                    let material = Dielectric::new(1.5);
                    let sphere = Sphere::from(center, 0.2, Rc::new(material)).unwrap();
                    world.add(Rc::new(sphere));
                }
            }
        }
    }

    let material1 = Dielectric::new(1.5);
    let material2 = Lambertian { albedo: Color::from_const(0.4, 0.2, 0.1) };
    let material3 = Metal { albedo: Color::from_const(0.7, 0.6, 0.5), fuzz: 0.0 };

    let sphere1 = Sphere::from(Point3::from_const(0.0, 1.0, 0.0), 1.0, Rc::new(material1)).unwrap();
    let sphere2 = Sphere::from(Point3::from_const(-4.0, 1.0, 0.0), 1.0, Rc::new(material2)).unwrap();
    let sphere3 = Sphere::from(Point3::from_const(4.0, 1.0, 0.0), 1.0, Rc::new(material3)).unwrap();

    world.add(Rc::new(sphere1));
    world.add(Rc::new(sphere2));
    world.add(Rc::new(sphere3));

    world
}
