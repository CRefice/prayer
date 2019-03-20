use nalgebra_glm as glm;

use super::*;

use crate::ray::Ray;
use crate::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Sphere { center, radius }
    }
}

impl Geometry for Sphere {
    fn intersection(&self, r: &Ray, min: f32, max: f32) -> Option<RayHit> {
        let oc = r.origin - self.center;
        let a = glm::dot(&r.direction, &r.direction);
        let b = glm::dot(&r.direction, &oc);
        let c = glm::dot(&oc, &oc) - self.radius * self.radius;
        let delta = b * b - a * c;
        if delta > 0.0 {
            let t = (-b - f32::sqrt(b * b - a * c)) / a;
            if t > min && t < max {
                let point = r.point_at(t);
                let normal = (point - self.center) / self.radius;
                return Some(RayHit {
                    t,
                    point,
                    normal
                });
            }
            let t = (-b + f32::sqrt(b * b - a * c)) / a;
            if t > min && t < max {
                let point = r.point_at(t);
                let normal = (point - self.center) / self.radius;
                Some(RayHit {
                    t,
                    point,
                    normal
                })
            } else {
                None
            }
        } else {
            None
        }
    }
}
