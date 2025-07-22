#[repr(C)]
#[derive(Clone, Debug)]
pub struct Colour {
	red: u8,
	green: u8,
	blue: u8,
	alpha: u8,
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
	pub const WHITE: Colour = Colour::new(255, 255, 255, 255);
	pub const BLACK: Colour = Colour::new(0, 0, 0, 0);
	pub const COLOURS: &[Colour] = &[
		Colour::RED,
		Colour::GREEN,
		Colour::TEAL,
		Colour::CYAN,
		Colour::BLUE,
		Colour::WHITE,
	];
}
