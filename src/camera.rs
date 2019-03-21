use nalgebra_glm as glm;

use crate::ray::Ray;
use crate::Vec3;

use std::f32::consts::PI;

pub struct Camera {
    position: Vec3,
    bl_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn looking_at(position: Vec3, at: Vec3, up: Vec3, fov: f32, aspect: f32) -> Self {
        let theta = fov * PI / 180.0;
        let half_h = f32::tan(theta / 2.0);
        let half_w = aspect * half_h;

        let w = glm::normalize(&(position - at));
        let u: Vec3 = glm::normalize(&w.cross(&up));
        let v = w.cross(&u);

        let bl_corner = position - half_w * u - half_h * v - w;
        let horizontal = 2.0 * half_w * u;
        let vertical = 2.0 * half_h * v;
        Camera {
            position,
            bl_corner,
            horizontal,
            vertical,
        }
    }

    pub fn ray_at(&self, x: f32, y: f32) -> Ray {
        Ray::new(
            self.position,
            self.bl_corner + x * self.horizontal + y * self.vertical - self.position,
        )
    }
}
