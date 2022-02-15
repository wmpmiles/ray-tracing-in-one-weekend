use serde::{Serialize, Deserialize};
use geometry3d::*;
use crate::object::List;
use crate::color::FloatRgb;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub image: ImageConfig,
    pub camera: CameraConfig,
    pub sampler: SamplerConfig,
    pub scene_list: List,
    pub background_color: FloatRgb,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageConfig {
    pub filename: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CameraConfig {
    pub look_from: Point3,
    pub look_at: Point3,
    pub up: Vec3,
    pub vertical_fov: f64,
    pub aperture: f64,
    pub focus_distance: f64,
    pub time_min: f64,
    pub time_max: f64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SamplerConfig {
    pub n: u32,
    pub max_depth: u32,
}

impl Config {
    pub fn read<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let config = serde_json::from_reader(reader)?;

        Ok(config)
    }
}

