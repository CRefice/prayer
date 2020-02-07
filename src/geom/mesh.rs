use std::path::Path;

use nalgebra_glm as glm;
use serde::{Deserialize, Deserializer};

use super::*;
use crate::obj;
use crate::ray::Ray;
use crate::{Vec2, Vec3};

#[derive(Clone)]
pub struct Vertex {
    pub pos: Vec3,
    pub normal: Vec3,
    pub uv: Vec2,
}

#[derive(Clone)]
pub struct Triangle {
    verts: [Vertex; 3],
}

pub struct Mesh {
    tree: KdTree<Triangle>,
}

impl Triangle {
    pub fn new(v1: Vertex, v2: Vertex, v3: Vertex) -> Self {
        Triangle {
            verts: [v1, v2, v3],
        }
    }

    pub fn positions(&self) -> (Vec3, Vec3, Vec3) {
        (self.verts[0].pos, self.verts[1].pos, self.verts[2].pos)
    }

    fn interpolate(&self, p: &Vec3) -> Vertex {
        let triangle_area = |e0: Vec3, e1: Vec3| glm::length(&e0.cross(&e1));
        let [v0, v1, v2] = &self.verts;
        let (p0, p1, p2) = self.positions();
        let f0 = p0 - p;
        let f1 = p1 - p;
        let f2 = p2 - p;
        let a = triangle_area(p1 - p0, p0 - p2);
        let a0 = triangle_area(f1, f2) / a;
        let a1 = triangle_area(f2, f0) / a;
        let a2 = triangle_area(f0, f1) / a;
        let uv = v0.uv * a0 + v1.uv * a1 + v2.uv * a2;
        let normal = v0.normal * a0 + v1.normal * a1 + v2.normal * a2;
        Vertex {
            pos: *p,
            uv,
            normal,
        }
    }
}

impl Geometry for Triangle {
    fn intersection(&self, r: &Ray, min: f32, max: f32) -> Option<RayHit> {
        let (v0, v1, v2) = self.positions();
        let e1 = v1 - v0;
        let e2 = v2 - v0;
        let pvec = r.direction.cross(&e2);
        let det = e1.dot(&pvec);

        // Cull backfaces by ignoring negative det
        if det.is_sign_positive() {
            let idet = 1.0 / det;
            let tvec = r.origin - v0;
            let qvec = tvec.cross(&e1);
            let t = e2.dot(&qvec) * idet;
            let uv = glm::vec2(tvec.dot(&pvec), r.direction.dot(&qvec)) * idet;
            if uv.x >= 0.0 && uv.x <= 1.0 && uv.y >= 0.0 && uv.x + uv.y <= 1.0 && t > min && t < max
            {
                let point = r.point_at(t);
                let Vertex { uv, normal, .. } = self.interpolate(&point);
                Some(RayHit {
                    t,
                    point,
                    normal,
                    uv,
                })
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl Bounds for Triangle {
    fn bounds(&self) -> AABB {
        AABB::from(self.verts.iter().map(|v| &v.pos))
    }
}

impl Mesh {
    pub fn from_file<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let tris = obj::load(path)?;
        let tree = KdTree::new(tris);
        Ok(Mesh { tree })
    }
}

impl Geometry for Mesh {
    fn intersection(&self, r: &Ray, min: f32, max: f32) -> Option<RayHit> {
        self.tree.intersection(r, min, max)
    }
}

impl Bounds for Mesh {
    fn bounds(&self) -> AABB {
        self.tree.bounds()
    }
}

impl<'de> Deserialize<'de> for Mesh {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        let mesh = Mesh::from_file(&s).map_err(serde::de::Error::custom)?;
        Ok(mesh)
    }
}
