use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use image::{self, hdr::HDRDecoder, Rgb};

pub struct HdrImage {
    buf: Vec<Rgb<f32>>,
    width: u32,
    height: u32,
}

impl HdrImage {
    pub fn get_pixel(&self, x: u32, y: u32) -> &Rgb<f32> {
        &self.buf[(y * self.width + x) as usize]
    }

    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
}

pub fn open<'a, P: AsRef<Path>>(path: P) -> Result<HdrImage, Box<dyn Error + 'a>> {
    let f = File::open(path)?;
    let reader = BufReader::new(f);
    let decoder = HDRDecoder::new(reader)?;
    let metadata = decoder.metadata();
    let width = metadata.width;
    let height = metadata.height;
    let buf = decoder.read_image_hdr()?;
    Ok(HdrImage { width, height, buf })
}
