use derive_more::{Add, Deref, DerefMut};

use crate::maths::vector::vector2::Vector2;

#[derive(Deref, DerefMut, Clone, Add, Copy)]
pub struct PixelCoordinate(Vector2<usize>);
impl PixelCoordinate {
	pub fn new(x: usize, y: usize) -> Self {
		Self(Vector2::new(x, y))
	}
}
