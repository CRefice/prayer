mod scene;
mod sphere;

use serde::{Deserialize, Serialize};

pub use self::scene::*;
pub use self::sphere::*;
use crate::material::Material;
use crate::ray::Ray;

use crate::Vec3;

pub trait Geometry {
    fn intersection(&self, ray: &Ray, min: f32, max: f32) -> Option<RayHit>;
}

pub trait Traceable {
    fn trace(&self, ray: &Ray, min: f32, max: f32) -> Option<TraceResult>;
}

pub struct RayHit {
    pub t: f32,
    pub point: Vec3,
    pub normal: Vec3,
}

#[derive(Serialize, Deserialize)]
pub struct Object {
    pub geometry: Sphere,
    pub material: Material,
}

pub struct TraceResult<'a> {
    pub hit: RayHit,
    pub material: &'a Material,
}

impl Object {
    pub fn new(geometry: Sphere, material: Material) -> Self {
        Object { geometry, material }
    }
}

impl Traceable for Object {
    fn trace(&self, ray: &Ray, min: f32, max: f32) -> Option<TraceResult> {
        self.geometry
            .intersection(ray, min, max)
            .map(|hit| TraceResult {
                hit,
                material: &self.material,
            })
    }
}
