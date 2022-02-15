use geometry3d::*;
use rtow::camera::Camera;
use rtow::color::*;
use rtow::config::Config;
use rtow::image::Image;
use rtow::object::*;
use rtow::sampler::SquareSampler;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut filename = "scene.json";
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        filename = &args[1];
    }

    let mut config = Config::read(filename)?;

    let mut image = Image::new(config.image);
    let camera = Camera::new(config.camera, &image);
    let sampler = SquareSampler::new(config.sampler, &image);
    let mut opt_scene = Object::from(BVHNode::from_list(
        &mut config.scene_list,
        TRange {
            start: camera.time_min,
            end: camera.time_max,
        },
    ));

    // using bottom left as (0,0)
    for (x, y) in image.iter() {
        let mut pixel_color = FRgbAccumulator::new();

        for (u, v) in sampler.iter(x, y) {
            let ray = camera.get_ray(u, v);
            pixel_color += ray_color(
                ray,
                config.background_color,
                &mut opt_scene,
                sampler.max_depth,
            );
        }

        image.add_pixel(pixel_color.average().into());
    }

    image.write()?;
    eprint!("\nDone.\n");

    Ok(())
}

fn ray_color(ray: Ray3, background: FloatRgb, world: &mut Object, depth: u32) -> FloatRgb {
    // minimize hitting the same point due to floating point approximation
    const RANGE: TRange<f64> = TRange {
        start: 0.001,
        end: f64::INFINITY,
    };

    if depth == 0 {
        FloatRgb::new(0.0, 0.0, 0.0)
    } else if let Some((rec, mat)) = world.hit(ray, RANGE) {
        let emitted = mat.emit(rec);
        if let Some((attenuation, ray)) = mat.scatter(rec) {
            emitted + attenuation * ray_color(ray, background, world, depth - 1)
        } else {
            emitted
        }
    } else {
        background
    }
}
