use maths::vector::vector3::Vector3;

use crate::graphics::{
	interpolate::Interpolate, shapes_2d::point::AbsoluteScreenCoordinate, shapes_3d::point::Point,
};

pub trait Shaders {
	/// Output of vertex shader
	type VsOut: Interpolate;
	type Pixel;
	type Vertex;

	fn vertex(&self, index: usize, vertex: Self::Vertex) -> Self::VsOut;

	fn fragment(&self, position: AbsoluteScreenCoordinate, data: Self::VsOut) -> Self::Pixel;
}
