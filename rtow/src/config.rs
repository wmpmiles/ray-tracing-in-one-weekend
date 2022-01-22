use std::str::FromStr;

pub struct Config {
    pub filename: String,
    pub image_width: u32,
    pub image_height: u32,
    pub aspect_ratio: f64,
    pub sampler_n: u32,
    pub max_depth: u32,
}

impl Config {
    fn arg_to_u32(args: &mut impl Iterator<Item = String>) -> u32 {
        u32::from_str(&args.next().unwrap()).unwrap()
    }

    pub fn parse(mut args: impl Iterator<Item = String>) -> Self {
        let mut filename = String::from(r"render.png");
        let mut image_width = 400;
        let mut image_height = 300;
        let mut sampler_n = 3;
        let mut max_depth = 50;

        while let Some(arg) = args.next() {
            match &arg[..] {
                "-f" => filename = args.next().unwrap(),
                "-w" => image_width = Self::arg_to_u32(&mut args),
                "-h" => image_height = Self::arg_to_u32(&mut args),
                "-n" => sampler_n = Self::arg_to_u32(&mut args),
                "-d" => max_depth = Self::arg_to_u32(&mut args),
                _ => (),
            }
        }

        let aspect_ratio = image_width as f64 / image_height as f64;

        Config {
            filename,
            image_width,
            image_height,
            aspect_ratio,
            sampler_n,
            max_depth,
        }
    }
}
