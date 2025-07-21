use pixels::Pixels;

use crate::{WIDTH, frame_pixels, graphics::colour::Colour, maths::vector::vector2::Vector2};

pub struct Screen<'a> {
	pub pixels: Pixels<'a>,
	pub draw_colour: Colour,
}
pub type Point = Vector2<usize>;
impl<'a> Screen<'a> {
	pub fn new(pixels: Pixels<'a>) -> Self {
		Self {
			pixels,
			draw_colour: Colour::new(0x48, 0xb2, 0xe8, 255),
		}
	}
	pub fn frame(&mut self) -> &mut [[Colour; WIDTH as usize]] {
		frame_pixels(self.pixels.frame_mut())
	}
	pub fn set_draw_colour(&mut self, colour: Colour) {
		self.draw_colour = colour;
	}
	pub fn draw_point(&mut self, point: Point) {
		self.frame()[point.y][point.x] = self.draw_colour.clone();
	}
	pub fn clear(&mut self, colour: Colour) {
		self.frame().as_flattened_mut().fill(colour);
	}
}
