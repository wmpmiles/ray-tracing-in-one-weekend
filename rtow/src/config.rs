use serde::{Serialize, Deserialize};
use geometry3d::*;

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
    pub aspect_ratio: f64,
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

