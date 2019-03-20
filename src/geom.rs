mod scene;
mod sphere;

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

pub struct Object<'a> {
    pub geometry: Box<dyn Geometry + 'a + Sync>,
    pub material: Material,
}

pub struct TraceResult<'a> {
    pub hit: RayHit,
    pub material: &'a Material,
}

impl<'a> Object<'a> {
    pub fn new(geometry: impl Geometry + 'a + Sync, material: Material) -> Self {
        Object {
            geometry: Box::new(geometry),
            material,
        }
    }
}

impl<'a> Traceable for Object<'a> {
    fn trace(&self, ray: &Ray, min: f32, max: f32) -> Option<TraceResult> {
        self.geometry
            .intersection(ray, min, max)
            .map(|hit| TraceResult {
                hit,
                material: &self.material,
            })
    }
}
