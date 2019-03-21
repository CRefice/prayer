use serde::{Deserialize, Serialize};

use super::*;
use crate::ray::Ray;

#[derive(Default, Serialize, Deserialize)]
pub struct Scene {
    objects: Vec<Object>,
}

impl Scene {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, obj: Object) {
        self.objects.push(obj)
    }
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
