use nalgebra_glm as glm;
use serde::{Deserialize, Serialize};

use super::*;

use crate::ray::Ray;
use crate::Vec3;

#[derive(Serialize, Deserialize)]
pub struct Plane {
    pub points: [Vec3; 4],
}

impl Plane {
    pub fn normal(&self) -> Vec3 {
        let side1 = self.points[1] - self.points[0];
        let side2 = self.points[3] - self.points[0];
        side1.cross(&side2).normalize()
    }

    pub fn contains(&self, point: Vec3) -> bool {
        let side1 = self.points[1] - self.points[0];
        let side2 = self.points[3] - self.points[0];
        let v = point - self.points[0];
        let width = glm::length(&side1);
        let height = glm::length(&side2);
        let proj1 = glm::dot(&v, &side1) / width;
        let proj2 = glm::dot(&v, &side2) / height;
        proj1 < width && proj1 > 0.0 && proj2 < height && proj2 > 0.0
    }
}

impl Geometry for Plane {
    fn intersection(&self, r: &Ray, min: f32, max: f32) -> Option<RayHit> {
        let normal = self.normal();
        let denom = glm::dot(&r.direction, &normal);
        if denom.abs() > 0.0001 {
            let num = glm::dot(&(self.points[0] - r.origin), &normal);
            let t = num / denom;
            let point = r.point_at(t);
            if self.contains(point) && t > min && t < max {
                let x = self.points[1] - self.points[0];
                let y = self.points[3] - self.points[0];
                let u = glm::dot(&x.normalize(), &(point - self.points[0])) / glm::length(&x);
                let v = glm::dot(&y.normalize(), &(point - self.points[0])) / glm::length(&y);
                let uv = glm::vec2(u, v);
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

impl Bounds for Plane {
    fn bounds(&self) -> AABB {
        AABB::from(self.points.iter())
    }
}
