use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use image::{hdr::HDRDecoder, Rgb};

use crate::Vec3;
use nalgebra_glm as glm;

pub struct Environment {
    data: Vec<Rgb<f32>>,
    width: usize,
    height: usize,
}

impl Environment {
    pub fn from_file(path: &Path) -> Result<Self, Box<dyn Error + '_>> {
        let f = File::open(path)?;
        let reader = BufReader::new(f);
        let decoder = HDRDecoder::new(reader)?;
        let metadata = decoder.metadata();
        let width = metadata.width as usize;
        let height = metadata.height as usize;
        let data = decoder.read_image_hdr()?;
        Ok(Environment {
            data,
            width,
            height,
        })
    }

    pub fn sample_direction(&self, dir: &Vec3) -> Vec3 {
        let u = 0.5 + f32::atan2(dir.z, dir.x) / (2.0 * std::f32::consts::PI);
        let v = 0.5 - f32::asin(dir.y) / std::f32::consts::PI;
        self.sample(u, v)
    }

    fn sample(&self, u: f32, v: f32) -> Vec3 {
        let x = u * (self.width as f32);
        let y = v * (self.height as f32);
        let (x1, y1) = (x.floor(), y.floor());
        let (x2, y2) = (x.ceil(), y.ceil());
        let f11 = self.pixel_at(x1, y1);
        let f21 = self.pixel_at(x2, y1);
        let f12 = self.pixel_at(x1, y2);
        let f22 = self.pixel_at(x2, y2);
        let q11 = f11 * (x2 - x) * (y2 - y);
        let q21 = f21 * (x - x1) * (y2 - y);
        let q12 = f12 * (x2 - x) * (y - y1);
        let q22 = f22 * (x - x1) * (y - y1);
        let num = q11 + q21 + q12 + q22;
        let denom = (x2 - x1) * (y2 - y1);
        num / denom;
        f11
    }

    fn pixel_at(&self, x: f32, y: f32) -> Vec3 {
        let idx = y as usize * self.width + x as usize;
        glm::make_vec3(&self.data[idx].data)
    }
}
