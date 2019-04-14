use std::error::Error;
use std::path::Path;

use serde::{de::Visitor, Deserialize, Deserializer};

use super::Texture;
use image::{self, GrayImage};

use crate::Vec2;
use nalgebra_glm as glm;

pub enum GrayScaleTexture {
    Tex(GrayImage),
    Solid(f32),
}

impl Texture for GrayScaleTexture {
    type Pixel = f32;

    fn dimensions(&self) -> Vec2 {
        match self {
            GrayScaleTexture::Tex(img) => glm::vec2(img.width() as f32, img.height() as f32),
            GrayScaleTexture::Solid(_color) => glm::vec2(100.0, 100.0),
        }
    }

    fn pixel_at(&self, x: u32, y: u32) -> Self::Pixel {
        match self {
            GrayScaleTexture::Tex(img) => f32::from(img.get_pixel(x, y).data[0]) / 255.0,
            GrayScaleTexture::Solid(color) => *color,
        }
    }
}

fn open<'a, P: AsRef<Path>>(path: P) -> Result<GrayScaleTexture, Box<dyn Error + 'a>> {
    let img = image::open(path)?;
    Ok(GrayScaleTexture::Tex(img.to_luma()))
}

impl<'de> Deserialize<'de> for GrayScaleTexture {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::Error;
        use std::fmt;

        struct TexVisitor;

        impl<'de> Visitor<'de> for TexVisitor {
            type Value = GrayScaleTexture;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("path to grayscale image file or solid value")
            }

            // Load from texture file
            fn visit_str<E: Error>(self, value: &str) -> Result<Self::Value, E> {
                open(value).map_err(E::custom)
            }

            fn visit_u64<E: Error>(self, val: u64) -> Result<Self::Value, E> {
                self.visit_f32(val as f32)
            }

            fn visit_i64<E: Error>(self, val: i64) -> Result<Self::Value, E> {
                self.visit_f32(val as f32)
            }

            fn visit_f64<E: Error>(self, val: f64) -> Result<Self::Value, E> {
                Ok(GrayScaleTexture::Solid(val as f32))
            }
        }

        deserializer.deserialize_any(TexVisitor)
    }
}
