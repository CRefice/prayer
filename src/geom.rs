mod aabb;
mod kdtree;
mod mesh;
mod plane;
mod scene;
mod sphere;

use serde::Deserialize;

pub use self::aabb::*;
pub use self::kdtree::*;
pub use self::mesh::*;
pub use self::plane::*;
pub use self::scene::*;
pub use self::sphere::*;

use crate::material::Material;
use crate::ray::Ray;

use crate::{Vec2, Vec3};

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
    pub uv: Vec2,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum GeomType {
    Sphere(Sphere),
    Plane(Plane),
    Mesh(Mesh),
}

impl Geometry for GeomType {
    fn intersection(&self, ray: &Ray, min: f32, max: f32) -> Option<RayHit> {
        match self {
            GeomType::Sphere(s) => s.intersection(ray, min, max),
            GeomType::Plane(p) => p.intersection(ray, min, max),
            GeomType::Mesh(m) => m.intersection(ray, min, max),
        }
    }
}

impl Bounds for GeomType {
    fn bounds(&self) -> AABB {
        match self {
            GeomType::Sphere(s) => s.bounds(),
            GeomType::Plane(p) => p.bounds(),
            GeomType::Mesh(m) => m.bounds(),
        }
    }
}

#[derive(Deserialize)]
pub struct Object {
    pub geometry: GeomType,
    pub material: Material,
}

pub struct TraceResult<'a> {
    pub hit: RayHit,
    pub material: &'a Material,
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
