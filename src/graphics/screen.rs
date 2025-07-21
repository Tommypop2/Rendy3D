use pixels::Pixels;

use crate::{WIDTH, frame_pixels, graphics::colour::Colour, maths::vector2::Vector2};

pub struct Screen<'a> {
	pub pixels: Pixels<'a>,
}

impl<'a> Screen<'a> {
	pub fn new(pixels: Pixels<'a>) -> Self {
		Self { pixels }
	}
	pub fn frame(&mut self) -> &mut [[Colour; WIDTH as usize]] {
		frame_pixels(self.pixels.frame_mut())
	}
	pub fn draw_point(&mut self, point: Vector2<usize>) {}
	pub fn clear(&mut self, colour: Colour) {
		self.frame()
			.as_flattened_mut()
			.fill(colour);
	}
}
