use std::path::Path;

use serde::de::Error;
use serde::{Deserialize, Deserializer};

use super::*;
use crate::environment::Environment;
use crate::ray::Ray;

#[derive(Deserialize)]
pub struct Scene {
    objects: Vec<Object>,

    #[serde(deserialize_with = "from_str")]
    pub environment: Environment,
}

impl Traceable for Scene {
    fn trace(&self, ray: &Ray, min: f32, max: f32) -> Option<TraceResult> {
        let mut max = max;
        let mut result = None;
        for obj in &self.objects {
            let traced = obj.trace(ray, min, max);
            if let Some(TraceResult { hit, .. }) = &traced {
                max = hit.t;
                result = traced;
            }
        }
        result
    }
}

fn from_str<'de, D>(deserializer: D) -> Result<Environment, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    Environment::from_file(Path::new(s)).map_err(D::Error::custom)
}
