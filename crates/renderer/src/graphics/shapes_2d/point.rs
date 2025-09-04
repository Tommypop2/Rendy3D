use derive_more::Add;

use crate::graphics::{draw::Draw, interpolate::Interpolate, pipeline::Pipeline, target::Target};

#[derive(Clone, Add, Copy, Debug)]
pub struct AbsoluteScreenCoordinate {
	pub x: usize,
	pub y: usize,
	pub z: f32,
}
#[cfg(feature = "std")]
use std::fmt::Display;
#[cfg(feature = "std")]
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
	fn draw<T: Target, P: Pipeline>(&self, target: &mut T, _pipeline: &mut P) {
		// Record Z in Z buffer if point is above Z buffer
		if !target.point_below_z_buffer(*self) {
			target.set_z_in_z_buffer(*self);
		} else {
			return;
		}
		target.draw_point(*self, target.draw_colour());
	}
}
