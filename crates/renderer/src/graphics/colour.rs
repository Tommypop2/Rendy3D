use derive_more::Mul;

use crate::graphics::interpolate::Interpolate;

#[repr(C)]
#[derive(Clone, Debug, Copy)]
pub struct Colour {
	pub red: u8,
	pub green: u8,
	pub blue: u8,
	pub alpha: u8,
}
impl Colour {
	pub const fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
		Self {
			red,
			green,
			blue,
			alpha,
		}
	}
}
impl Default for Colour {
	fn default() -> Self {
		Self::new(255, 255, 255, 255)
	}
}
impl Colour {
	pub const RED: Colour = Colour::new(255, 0, 0, 255);
	pub const GREEN: Colour = Colour::new(0, 255, 0, 255);
	pub const TEAL: Colour = Colour::new(0, 128, 128, 255);
	pub const CYAN: Colour = Colour::new(0, 255, 255, 255);
	pub const BLUE: Colour = Colour::new(0, 0, 255, 255);
	pub const PURPLE: Colour = Colour::new(138, 43, 226, 255);
	pub const WHITE: Colour = Colour::new(255, 255, 255, 255);
	pub const BLACK: Colour = Colour::new(0, 0, 0, 0);
	pub const COLOURS: &[Colour] = &[
		Colour::RED,
		Colour::GREEN,
		Colour::TEAL,
		Colour::CYAN,
		Colour::BLUE,
		Colour::PURPLE,
		Colour::WHITE,
	];
}
impl Interpolate for Colour {
	fn interpolate3(a: &Self, b: &Self, c: &Self, x: f32, y: f32, z: f32) -> Self {
		Colour::new(
			(a.red as f32 * x + b.red as f32 * y + c.red as f32 * z) as u8,
			(a.green as f32 * x + b.green as f32 * y + c.green as f32 * z) as u8,
			(a.blue as f32 * x + b.blue as f32 * y + c.blue as f32 * z) as u8,
			(a.alpha as f32 * x + b.alpha as f32 * y + c.alpha as f32 * z) as u8,
		)
	}
}
