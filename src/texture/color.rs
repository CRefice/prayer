use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use image::{self, hdr::HDRDecoder};

use serde::{de::Visitor, Deserialize, Deserializer};

use super::Texture;

use crate::{Vec2, Vec3};
use nalgebra_glm as glm;

pub struct ColorTexture {
    buf: Vec<Vec3>,
    width: u32,
    height: u32,
}

impl ColorTexture {
    pub fn solid(color: Vec3) -> Self {
        ColorTexture {
            buf: vec![color],
            width: 1,
            height: 1,
        }
    }
}

impl Default for ColorTexture {
    fn default() -> Self {
        Self::solid(Vec3::new(0.0, 0.0, 0.0))
    }
}

impl Texture for ColorTexture {
    type Pixel = Vec3;

    fn dimensions(&self) -> Vec2 {
        glm::vec2(self.width as f32, self.height as f32)
    }

    fn pixel_at(&self, x: u32, y: u32) -> Self::Pixel {
        let idx = (y * self.width + x) as usize;
        self.buf[idx]
    }
}

fn open<'a, P: AsRef<Path>>(path: P) -> Result<ColorTexture, Box<dyn Error + 'a>> {
    use std::ffi::OsStr;
    if let Some("hdr") = path.as_ref().extension().and_then(OsStr::to_str) {
        open_hdr(path)
    } else {
        let img = image::open(path)?.to_rgb();
        let (width, height) = img.dimensions();
        let buf = img.pixels().map(|p| rgb_to_float(*p)).collect();
        Ok(ColorTexture { buf, width, height })
    }
}

fn open_hdr<'a, P: AsRef<Path>>(path: P) -> Result<ColorTexture, Box<dyn Error + 'a>> {
    let f = File::open(path)?;
    let reader = BufReader::new(f);
    let decoder = HDRDecoder::new(reader)?;
    let metadata = decoder.metadata();
    let width = metadata.width;
    let height = metadata.height;
    let buf = decoder
        .read_image_hdr()?
        .into_iter()
        .map(|pix| glm::make_vec3(&pix.data))
        .collect();
    Ok(ColorTexture { width, height, buf })
}

fn rgb_to_float(pix: image::Rgb<u8>) -> Vec3 {
    let [r, g, b] = pix.data;
    let vec = Vec3::new(
        f32::from(r) / 255.0,
        f32::from(g) / 255.0,
        f32::from(b) as f32 / 255.0,
    );
    glm::pow(&vec, &glm::vec3(2.2, 2.2, 2.2))
}

impl<'de> Deserialize<'de> for ColorTexture {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{value::SeqAccessDeserializer, Error, SeqAccess};
        use std::fmt;

        struct TexVisitor;

        impl<'de> Visitor<'de> for TexVisitor {
            type Value = ColorTexture;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("path to color image file or array")
            }

            // Load from texture file
            fn visit_str<E: Error>(self, value: &str) -> Result<Self::Value, E> {
                open(value).map_err(E::custom)
            }

            // Solid color
            fn visit_seq<A: SeqAccess<'de>>(self, value: A) -> Result<Self::Value, A::Error> {
                let color: Vec3 = Deserialize::deserialize(SeqAccessDeserializer::new(value))?;
                Ok(ColorTexture::solid(color))
            }
        }

        deserializer.deserialize_any(TexVisitor)
    }
}
