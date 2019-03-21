use std::error::Error;
use std::fs;
use std::path::Path;

use nalgebra_glm::UVec2;
use serde::{Deserialize, Serialize};

use crate::geom::Scene;

#[derive(Serialize, Deserialize)]
pub struct RenderParams {
    pub resolution: UVec2,
    pub samples: usize,
    pub max_light_bounces: usize,
    pub gamma: f32,
}

#[derive(Serialize, Deserialize)]
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
