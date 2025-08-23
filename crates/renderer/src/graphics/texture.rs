use std::path::Path;

use image::{DynamicImage, GenericImageView, ImageBuffer, ImageReader, Rgba};

use crate::graphics::colour::Colour;

pub struct Texture {
	base: DynamicImage,
}
impl Texture {
	pub const fn new(base: DynamicImage) -> Self {
		Self { base }
	}
	pub fn from_path<P: AsRef<Path>>(path: P) -> Self {
		let img = ImageReader::open(path).unwrap().decode().unwrap();
		Self::new(img)
	}
	pub fn get_pixel(&self, u: f32, v: f32) -> Colour {
		let (width, height) = self.base.dimensions();
		let x = (width as f32 * u) as u32;
		let y = (height as f32 * v) as u32;
		self.base.get_pixel(x % width, y % height).into()
	}
}
impl From<Rgba<u8>> for Colour {
	fn from(value: Rgba<u8>) -> Self {
		let c = value.0;
		Colour::new(c[0], c[1], c[2], c[3])
	}
}
