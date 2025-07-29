use std::ops::{Mul, Sub};

use crate::vector::vector2::Vector2;

#[derive(Clone, Copy)]
pub struct Matrix2<T> {
	x: Vector2<T>,
	y: Vector2<T>,
}

impl<T> Matrix2<T> {
	pub fn new(x: Vector2<T>, y: Vector2<T>) -> Self {
		Self { x, y }
	}
}

impl<T> Matrix2<T>
where
	T: Mul<Output = T> + Sub<Output = T> + Clone + Copy,
{
	pub fn determinant(&self) -> T {
		// ad - bc
		(self.x.x * self.y.y) - (self.y.x * self.x.y)
	}
}
