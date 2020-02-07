use std::error::Error;
use std::fs;
use std::path::Path;

use nalgebra_glm::{zero, UVec2};
use serde::Deserialize;

use crate::geom::Scene;
use crate::Vec3;

#[derive(Deserialize)]
#[serde(default)]
pub struct RenderParams {
    pub resolution: UVec2,
    pub samples: usize,
    pub max_light_bounces: usize,
    pub gamma: f32,
    pub exposure: f32,
    pub camera_pos: Vec3,
    pub looking_at: Vec3,
    pub fov: f32,
}

impl Default for RenderParams {
    fn default() -> Self {
        RenderParams {
            resolution: UVec2::new(500, 500),
            samples: 10,
            max_light_bounces: 5,
            gamma: 2.2,
            exposure: 1.0,
            camera_pos: Vec3::new(0.0, 0.0, -1.0),
            looking_at: zero(),
            fov: 80.0,
        }
    }
}

#[derive(Deserialize)]
pub struct UserConfig {
    pub params: RenderParams,
    pub scene: Scene,
}

impl UserConfig {
    pub fn from_file(path: &Path) -> Result<Self, Box<dyn Error + '_>> {
        let contents = fs::read_to_string(path)?;
        let cfg = toml::from_str(&contents)?;
        Ok(cfg)
    }
}
