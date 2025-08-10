use std::fmt::Display;

use derive_more::Add;

use crate::graphics::{draw::Draw, interpolate::Interpolate, shaders::shaders::Shaders, target::Target};

#[derive(Clone, Add, Copy)]
pub struct AbsoluteScreenCoordinate {
	pub x: usize,
	pub y: usize,
	pub z: f32,
}
impl Display for AbsoluteScreenCoordinate {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		writeln!(f, "({}, {}, {})", self.x, self.y, self.z)
	}
}
impl AbsoluteScreenCoordinate {
	pub fn new(x: usize, y: usize, z: f32) -> Self {
		Self { x, y, z }
	}
	pub fn as_tuple(self) -> (usize, usize, f32) {
		(self.x, self.y, self.z)
	}
}

pub static mut MAX_Z: f32 = 0.0;
impl<VsOut: Interpolate> Draw<VsOut> for AbsoluteScreenCoordinate {
	fn draw<T: Target, S: Shaders>(&self, target: &mut T, shaders: &mut S) {
		// Record Z in Z buffer if point is above Z buffer
		if !target.point_below_z_buffer(*self) {
			target.set_z_in_z_buffer(*self);
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
		// let (r,g,b) = hsv_to_rgb((self.z * 360.0).clamp(0.0, 360.0) as f64 * 0.75, 1.0, 1.0);
		// screen.set_draw_colour(Colour::new(r, g, b, 255));
		target.draw_point(*self, target.draw_colour());
	}
}
