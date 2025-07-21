use pixels::Pixels;

use crate::{
	WIDTH, frame_pixels,
	graphics::{colour::Colour, shapes::triangle::Draw},
	maths::vector::vector2::Vector2,
};

pub struct Screen<'a> {
	pub pixels: Pixels<'a>,
	draw_colour: Colour,
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
	fn draw_line_low(&mut self, start: Point, end: Point) {
		let dx = (end.x - start.x) as i32;
		let (dy, yi) = {
			let dy = end.y as i32 - start.y as i32;
			if dy < 0 { (-dy, -1) } else { (dy, 1) }
		};
		let mut d = 2 * dy - dx;
		let mut y = start.y as i32;
		for x in start.x..=end.x {
			self.draw_point(Vector2::new(x, y as usize));
			if d > 0 {
				y += yi;
				d = d + (2 * (dy - dx))
			} else {
				d = d + 2 * dy
			}
		}
	}
	fn draw_line_high(&mut self, start: Point, end: Point) {
		let (dx, xi) = {
			let dx = end.x as i32 - start.x as i32;
			if dx < 0 { (-dx, -1) } else { (dx, 1) }
		};
		let dy = (end.y - start.y) as i32;
		let mut d = 2 * dx - dy;
		let mut x = start.x as i32;
		for y in start.y..=end.y {
			self.draw_point(Vector2::new(x as usize, y));
			if d > 0 {
				x += xi;
				d = d + (2 * (dx - dy))
			} else {
				d = d + 2 * dx
			}
		}
	}
	pub fn draw_line(&mut self, start: Point, end: Point) {
		if usize::abs_diff(start.y, end.y) < usize::abs_diff(start.x, end.x) {
			if end.x > start.x {
				self.draw_line_low(start, end);
			} else {
				self.draw_line_low(end, start);
			}
		} else {
			if end.y > start.y {
				self.draw_line_high(start, end);
			} else {
				self.draw_line_high(end, start);
			}
		}
	}
	pub fn draw_shape<T: Draw>(&mut self, shape: T) {
		shape.draw(self);
	}
	pub fn clear(&mut self, colour: Colour) {
		self.frame().as_flattened_mut().fill(colour);
	}
}
