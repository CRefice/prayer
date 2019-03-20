use super::*;

use crate::ray::Ray;

#[derive(Default)]
pub struct Scene<'a> {
    objects: Vec<Object<'a>>,
}

impl<'a> Scene<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, obj: Object<'a>) {
        self.objects.push(obj)
    }
}

impl<'a> Traceable for Scene<'a> {
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
