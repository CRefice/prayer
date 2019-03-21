use nalgebra_glm as glm;
use serde::{Deserialize, Serialize};

use rand::prelude::*;

use crate::geom::RayHit;
use crate::ray::Ray;
use crate::Vec3;

fn transform_to_world(vec: &Vec3, norm: &Vec3) -> Vec3 {
    // Find an axis that is not parallel to normal
    let major_axis = if f32::abs(norm.x) < (1.0 / f32::sqrt(3.0)) {
        glm::vec3(1.0, 0.0, 0.0)
    } else if f32::abs(norm.y) < (1.0 / f32::sqrt(3.0)) {
        glm::vec3(0.0, 1.0, 0.0)
    } else {
        glm::vec3(0.0, 0.0, 1.0)
    };

    // Create a coordinate system relative to world space
    let u = glm::normalize(&norm.cross(&major_axis));
    let v = norm.cross(&u);
    let w = norm;

    // Transform from local coordinates to world coordinates
    v * vec.x + w * vec.y + u * vec.z
}

#[derive(Serialize, Deserialize)]
pub struct Material {
    pub color: Vec3,
    pub metalness: f32,
    pub roughness: f32,
    pub emission: Vec3,
}

impl Material {
    fn importance_theta(&self) -> f32 {
        let mut rng = rand::thread_rng();
        let a = self.roughness * self.roughness;
        let eta: f32 = rng.gen();
        let sqrt = f32::sqrt(eta / (1.0 - eta));
        f32::atan(a * sqrt)
    }

    pub fn bounce(&self, w0: &Vec3, hit: &RayHit) -> (Ray, f32) {
        let n = hit.normal;
        let mut rng = rand::thread_rng();
        let theta = self.importance_theta();
        let phi: f32 = rng.gen::<f32>() * 2.0 * std::f32::consts::PI;

        let x = f32::sin(theta) * f32::sin(phi);
        let y = f32::cos(theta);
        let z = f32::sin(theta) * f32::cos(phi);

        let direction = glm::normalize(&transform_to_world(&glm::vec3(x, y, z), &n));
        let h = glm::normalize(&(w0 + direction));

        let cost = f32::max(0.0, glm::dot(&n, &h));
        let pdf = normal_distribution(&n, &h, self.roughness) * cost;
        let p = pdf / (4.0 * f32::max(0.0, glm::dot(&w0, &h)));
        (Ray::new(hit.point, direction), p)
    }

    /// Return type is (brdf, fresnel)
    pub fn brdf(&self, w0: &Vec3, wi: &Vec3, n: &Vec3) -> (Vec3, Vec3) {
        let h = glm::normalize(&(w0 + wi));
        let d = normal_distribution(&n, &h, self.roughness);
        let f0 = glm::vec3(0.04, 0.04, 0.04);
        let f0 = glm::mix(&f0, &self.color, self.metalness);
        let f = fresnel(&wi, &h, &f0);
        let g = geometry(&n, &h, w0, wi);
        let num = d * f * g;
        let denom = 4.0 * glm::dot(&n, &wi) * glm::dot(&n, &w0);
        (num / denom, f)
    }
}

fn normal_distribution(n: &Vec3, h: &Vec3, roughness: f32) -> f32 {
    let a = roughness * roughness;
    let ndoth = f32::max(glm::dot(n, h), 0.0);
    let num = a * a;
    let denom = (ndoth * ndoth) * (a * a - 1.0) + 1.0;
    let denom = std::f32::consts::PI * denom * denom;
    num / denom
}

fn fresnel(wi: &Vec3, h: &Vec3, f0: &Vec3) -> Vec3 {
    let widoth = f32::max(0.0, glm::dot(wi, h));
    f0 + (glm::vec3(1.0, 1.0, 1.0) - f0) * f32::powi(1.0 - widoth, 5)
}

fn geometry(n: &Vec3, h: &Vec3, w0: &Vec3, wi: &Vec3) -> f32 {
    let ndoth = f32::max(0.0, glm::dot(n, h));
    let w0doth = f32::max(0.0, glm::dot(w0, h));
    let term2 = 2.0 * ndoth * f32::max(0.0, glm::dot(n, w0)) / w0doth;
    let term3 = 2.0 * ndoth * f32::max(0.0, glm::dot(n, wi)) / w0doth;
    f32::min(1.0, f32::min(term2, term3))
}

impl Default for Material {
    fn default() -> Self {
        Material {
            color: Vec3::new(1.0, 1.0, 1.0),
            metalness: 0.0,
            roughness: 1.0,
            emission: Vec3::new(0.0, 0.0, 0.0),
        }
    }
}
