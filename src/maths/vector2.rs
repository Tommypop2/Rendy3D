use std::ops::{Add, Mul};

use crate::maths::SqrtAcos;

pub struct Vector2<T: Mul<Output = T> + Add<Output = T> + Copy> {
	x: T,
	y: T,
}

impl<T: Mul<Output = T> + Add<Output = T> + Copy> Vector2<T> {
	pub fn new(x: T, y: T) -> Self {
		Self { x, y }
	}
	pub fn magnitude_squared(&self) -> T {
		self.x * self.x + self.y * self.y
	}
}

impl<T: Mul<Output = T> + Add<Output = T> + Copy + SqrtAcos> Vector2<T> {
	pub fn magnitude(&self) -> T {
		self.magnitude_squared().sqrt()
	}
}
