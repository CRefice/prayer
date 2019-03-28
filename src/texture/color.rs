use std::error::Error;
use std::path::Path;

use serde::{de::Visitor, Deserialize, Deserializer};

use super::{hdr, Texture};
use image::{self, RgbImage};

use crate::{Vec2, Vec3};
use nalgebra_glm as glm;

pub enum ColorTexture {
    Dyn(RgbImage),
    Hdr(hdr::HdrImage),
    Solid(Vec3),
}

impl Texture for ColorTexture {
    type Pixel = Vec3;

    fn dimensions(&self) -> Vec2 {
        match self {
            ColorTexture::Dyn(img) => glm::vec2(img.width() as f32, img.height() as f32),
            ColorTexture::Hdr(img) => glm::vec2(img.width() as f32, img.height() as f32),
            ColorTexture::Solid(_color) => glm::vec2(100.0, 100.0),
        }
    }

    fn pixel_at(&self, x: u32, y: u32) -> Self::Pixel {
        match self {
            ColorTexture::Dyn(img) => rgb_to_float(img.get_pixel(x, y)),
            ColorTexture::Hdr(img) => glm::make_vec3(&img.get_pixel(x, y).data),
            ColorTexture::Solid(color) => color.clone(),
        }
    }
}

fn open<'a, P: AsRef<Path>>(path: P) -> Result<ColorTexture, Box<dyn Error + 'a>> {
    use std::ffi::OsStr;
    if let Some(".hdr") = path.as_ref().extension().and_then(OsStr::to_str) {
        let img = hdr::open(path)?;
        Ok(ColorTexture::Hdr(img))
    } else {
        let img = image::open(path)?;
        Ok(ColorTexture::Dyn(img.to_rgb()))
    }
}

pub fn rgb_to_float(pix: &image::Rgb<u8>) -> Vec3 {
    let [r, g, b] = pix.data;
    Vec3::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
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
                let vec: Vec3 = Deserialize::deserialize(SeqAccessDeserializer::new(value))?;
                Ok(ColorTexture::Solid(vec))
            }
        }

        deserializer.deserialize_any(TexVisitor)
    }
}
