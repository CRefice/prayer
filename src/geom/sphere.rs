use nalgebra_glm as glm;
use serde::{Deserialize, Serialize};

use super::*;

use crate::ray::Ray;
use crate::Vec3;

#[derive(Serialize, Deserialize)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
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
                let uv = Self::uv_at_dir(&(point - self.center).normalize());
                return Some(RayHit {
                    t,
                    point,
                    normal,
                    uv,
                });
            }
            let t = (-b + f32::sqrt(b * b - a * c)) / a;
            if t > min && t < max {
                let point = r.point_at(t);
                let normal = (point - self.center) / self.radius;
                let uv = Self::uv_at_dir(&(point - self.center).normalize());
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

impl Bounds for Sphere {
    fn bounds(&self) -> AABB {
        let r_vec = glm::vec3(self.radius, self.radius, self.radius);
        let min = self.center - r_vec;
        let max = self.center + r_vec;
        AABB { min, max }
    }
}

impl Sphere {
    pub fn uv_at_dir(dir: &Vec3) -> Vec2 {
        let u = 0.5 + f32::atan2(dir.z, dir.x) / glm::two_pi::<f32>();
        let v = 0.5 - f32::asin(dir.y) / glm::pi::<f32>();
        Vec2::new(u, v)
    }
}
