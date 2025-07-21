#[repr(C)]
#[derive(Clone, Debug)]
pub struct Colour {
	red: u8,
	green: u8,
	blue: u8,
	alpha: u8,
}
impl Colour {
	pub fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
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
