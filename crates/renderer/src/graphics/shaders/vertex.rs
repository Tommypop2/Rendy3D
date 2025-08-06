use maths::vector::vector3::Vector3;

use crate::graphics::{colour::Colour, shapes_3d::point::Point};

pub struct VertexShader<T> {
	data: T,
	shader: fn(data: &mut T, index: usize, vertex: Point, normal: Vector3<f64>) -> Colour,
}

impl<T> VertexShader<T> {
	pub const fn new(
		data: T,
		shader: fn(data: &mut T, index: usize, vertex: Point, normal: Vector3<f64>) -> Colour,
	) -> Self {
		Self { data, shader }
	}
	pub fn execute(&mut self, index: usize, vertex: Point, normal: Vector3<f64>) -> Colour {
		(self.shader)(&mut self.data, index, vertex, normal)
	}
}
