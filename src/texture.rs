mod color;
mod grayscale;

use std::ops::*;

use crate::Vec2;
use nalgebra_glm as glm;

pub use color::*;
pub use grayscale::*;

pub trait Texture {
    type Pixel: Mul<f32, Output = Self::Pixel> + Add<Self::Pixel, Output = Self::Pixel>;

    fn dimensions(&self) -> Vec2;

    fn pixel_at(&self, x: u32, y: u32) -> Self::Pixel;

    fn sample(&self, uv: &Vec2) -> Self::Pixel {
        let dim = self.dimensions();
        let point = uv.component_mul(&(dim - glm::vec2(1.0, 1.0)));
        let (p1, p2) = (glm::floor(&point), glm::ceil(&point));
        let t = point - p1;
        let f11 = self.pixel_at(p1.x as u32, p1.y as u32);
        let f21 = self.pixel_at(p2.x as u32, p1.y as u32);
        let f12 = self.pixel_at(p1.x as u32, p2.y as u32);
        let f22 = self.pixel_at(p2.x as u32, p2.y as u32);
        let a = f11 * (1.0 - t.x) + f21 * t.x;
        let b = f12 * (1.0 - t.x) + f22 * t.x;
        a * (1.0 - t.y) + b * t.y
    }
}
