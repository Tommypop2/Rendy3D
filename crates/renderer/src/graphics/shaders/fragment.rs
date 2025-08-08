use maths::vector::vector3::Vector3;

use crate::graphics::{colour::Colour, shapes_2d::point::AbsoluteScreenCoordinate};

pub struct FragmentShader<DATA, RETURN = Colour> {
	data: DATA,
	shader:
		fn(data: &mut DATA, point: AbsoluteScreenCoordinate, barycentric: Vector3<f32>) -> RETURN,
}

impl<DATA, RETURN> FragmentShader<DATA, RETURN> {
	pub const fn new(
		data: DATA,
		shader: fn(
			data: &mut DATA,
			point: AbsoluteScreenCoordinate,
			barycentric: Vector3<f32>,
		) -> RETURN,
	) -> Self {
		Self { data, shader }
	}
	pub fn execute(
		&mut self,
		point: AbsoluteScreenCoordinate,
		barycentric: Vector3<f32>,
	) -> RETURN {
		(self.shader)(&mut self.data, point, barycentric)
	}
}
