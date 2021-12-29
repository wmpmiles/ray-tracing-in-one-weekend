use rtow::camera::Camera;
use rtow::color::Color;
use rtow::material::*;
use rtow::object::*;
use rtow::ray::Ray;
use rtow::vec3::*;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::rc::Rc;

fn main() {
    // Image
    const FILENAME: &str = r"fullrender4.png";
    const IMAGE_WIDTH: u32 = 1920;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / Camera::ASPECT_RATIO) as u32;
    const IMAGE_SIZE: usize = (IMAGE_WIDTH * IMAGE_HEIGHT * 4) as usize;

    const SAMPLES_PER_PIXEL: u32 = 100;
    const MAX_DEPTH: u32 = 50;

    let path = Path::new(FILENAME);
    let file = File::create(path).unwrap();
    let w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, IMAGE_WIDTH, IMAGE_HEIGHT);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    let mut data: Vec<u8> = Vec::with_capacity(IMAGE_SIZE);

    // World
    let mut world = ObjectList::new();

    let material_ground: Rc<dyn Material> =
        Rc::new(Lambertian::new(Color::from_const(0.8, 0.8, 0.0)));
    let material_centre: Rc<dyn Material> =
        Rc::new(Lambertian::new(Color::from_const(0.1, 0.2, 0.5)));
    let material_left: Rc<dyn Material> = Rc::new(Dielectric::new(1.5));
    let material_right: Rc<dyn Material> =
        Rc::new(Metal::new(Color::from_const(0.8, 0.6, 0.2), 0.0));

    world.add(Rc::new(Sphere::from(
        Point3::from(0.0, -100.5, -1.0),
        100.0,
        Rc::clone(&material_ground),
    )));
    world.add(Rc::new(Sphere::from(
        Point3::from(0.0, 0.0, -1.0),
        0.5,
        Rc::clone(&material_centre),
    )));
    world.add(Rc::new(Sphere::from(
        Point3::from(-1.0, 0.0, -1.0),
        0.5,
        Rc::clone(&material_left),
    )));
    world.add(Rc::new(Sphere::from(
        Point3::from(-1.0, 0.0, -1.0),
        -0.4,
        Rc::clone(&material_left),
    )));
    world.add(Rc::new(Sphere::from(
        Point3::from(1.0, 0.0, -1.0),
        0.5,
        Rc::clone(&material_right),
    )));

    // Camera
    let camera = Camera::new();

    // Render
    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new();

            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (rand::random::<f64>() + i as f64) / (IMAGE_WIDTH - 1) as f64;
                let v = (rand::random::<f64>() + j as f64) / (IMAGE_HEIGHT - 1) as f64;

                let ray = camera.get_ray(u, v);

                pixel_color += ray_color(&ray, &world, MAX_DEPTH);
            }

            pixel_color.write(SAMPLES_PER_PIXEL, &mut data);
        }
    }

    writer.write_image_data(&data).unwrap();
    eprint!("\nDone.\n");
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
        let unit_direction = ray.direction.unit_vector();
        let t = 0.5 * (unit_direction.1 + 1.0);

        ONE.scalar_mul(1.0 - t) + BASE.scalar_mul(t)
    }
}
