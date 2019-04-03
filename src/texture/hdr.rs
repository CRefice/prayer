use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use image::{self, hdr::HDRDecoder};
use nalgebra_glm as glm;

use super::ColorTexture;

pub fn open<'a, P: AsRef<Path>>(path: P) -> Result<ColorTexture, Box<dyn Error + 'a>> {
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
