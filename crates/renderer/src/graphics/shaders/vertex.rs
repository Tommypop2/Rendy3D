use maths::vector::vector3::Vector3;

use crate::graphics::{colour::Colour, shapes_3d::point::Point};

pub struct VertexShader<T, PIXEL = Colour> {
	data: T,
	shader: fn(data: &mut T, index: usize, vertex: Point, normal: Vector3<f64>) -> PIXEL,
}

impl<T, PIXEL> VertexShader<T, PIXEL> {
	pub const fn new(
		data: T,
		shader: fn(data: &mut T, index: usize, vertex: Point, normal: Vector3<f64>) -> PIXEL,
	) -> Self {
		Self { data, shader }
	}
	pub fn execute(&mut self, index: usize, vertex: Point, normal: Vector3<f64>) -> PIXEL {
		(self.shader)(&mut self.data, index, vertex, normal)
	}
}
