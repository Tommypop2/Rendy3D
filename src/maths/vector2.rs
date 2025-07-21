use std::ops::{Add, Mul};

use crate::maths::SqrtAcos;

#[derive(Debug, Clone, Copy)]
pub struct Vector2<T: Mul<Output = T> + Add<Output = T> + Copy> {
	pub x: T,
	pub y: T,
}

impl<T: Mul<Output = T> + Add<Output = T> + Copy> Vector2<T> {
	pub fn new(x: T, y: T) -> Self {
		Self { x, y }
	}
	pub fn magnitude_squared(&self) -> T {
		self.x * self.x + self.y * self.y
	}
	pub fn as_tuple(&self) -> (T, T) {
		(self.x, self.y)
	}
}

impl<T: Mul<Output = T> + Add<Output = T> + Copy + SqrtAcos> Vector2<T> {
	pub fn magnitude(&self) -> T {
		self.magnitude_squared().sqrt()
	}
}
