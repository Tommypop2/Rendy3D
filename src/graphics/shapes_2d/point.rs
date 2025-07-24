use derive_more::Add;
use hsv::hsv_to_rgb;

use crate::{
	graphics::{colour::Colour, shapes_2d::triangle::Draw, shapes_3d::point::Point}, HEIGHT, WIDTH
};

#[derive(Clone, Add, Copy)]
pub struct PixelCoordinate {
	pub x: usize,
	pub y: usize,
	pub z: f32,
}
impl PixelCoordinate {
	pub fn new(x: usize, y: usize, z: f32) -> Self {
		Self { x, y, z }
	}
	pub fn as_tuple(self) -> (usize, usize, f32) {
		(self.x, self.y, self.z)
	}
}

impl From<Point> for PixelCoordinate {
	fn from(value: Point) -> Self {
		let offset = PixelCoordinate::new((WIDTH / 2) as usize, (HEIGHT / 2) as usize, 0.0);
		let x = (offset.x as f64 + value.x * (WIDTH as f64) / 100.0).round() as usize;
		let y = (offset.y as f64 + value.y * (HEIGHT as f64) / 100.0).round() as usize;
		Self::new(x, y, value.z as f32)
	}
}
pub static mut MAX_Z: f32 = 0.0;
impl Draw for PixelCoordinate {
	fn draw(
		&self,
		viewport: &mut crate::graphics::viewport::Viewport,
		screen: &mut crate::graphics::screen::Screen,
	) {
		// Record Z in Z buffer if point is above Z buffer
		if !viewport.point_below_z_buffer(screen, *self) {
			screen.z_buffer[self.y][self.x] = self.z;
		} else {
			return;
		}
		// screen.frame()[self.y][self.x] = screen.draw_colour.clone();
		// unsafe {
		// 	if (self.z > MAX_Z) {
		// 		MAX_Z = self.z
		// 	}
		// };
		// let z_normalised = self.z / (56.241528 * 2.0) + 0.5;
		// let (r,g,b) = hsv_to_rgb((z_normalised * 360.0).clamp(0.0, 360.0) as f64 * 0.75, 1.0, 1.0);
		// screen.set_draw_colour(Colour::new(r, g, b, 255));
		viewport.draw_point(screen, *self);
	}
}
