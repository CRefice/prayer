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
        let u = 0.5 + f32::atan2(dir.z, dir.x) / glm::two_pi::<f32>();
        let v = 0.5 - f32::asin(dir.y) / glm::pi::<f32>();
        self.sample(u, v)
    }

    fn sample(&self, u: f32, v: f32) -> Vec3 {
        assert!(u <= 1.0 && v <= 1.0);
        let x = u * (self.width - 1) as f32;
        let y = v * (self.height - 1) as f32;
        let (x1, y1) = (x.floor(), y.floor());
        let (x2, y2) = (x.ceil(), y.ceil());
        let (tx, ty) = (x - x1, y - y1);
        let f11 = self.pixel_at(x1, y1);
        let f21 = self.pixel_at(x2, y1);
        let f12 = self.pixel_at(x1, y2);
        let f22 = self.pixel_at(x2, y2);
        let a = f11 * (1.0 - tx) + f21 * tx;
        let b = f12 * (1.0 - tx) + f22 * tx;
        a * (1.0 - ty) + b * ty
    }

    fn pixel_at(&self, x: f32, y: f32) -> Vec3 {
        let idx = y as usize * self.width + x as usize;
        glm::make_vec3(&self.data[idx].data)
    }
}
