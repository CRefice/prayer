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
        let side1 = self.points[0] - self.points[1];
        let side2 = self.points[0] - self.points[3];
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
                Some(RayHit { t, point, normal })
            } else {
                None
            }
        } else {
            None
        }
    }
}
