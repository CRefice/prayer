use serde::Deserialize;

use super::*;
use crate::ray::Ray;
use crate::texture::ColorTexture;

#[derive(Deserialize)]
pub struct Scene {
    objects: Vec<Object>,
    pub environment: ColorTexture,
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
