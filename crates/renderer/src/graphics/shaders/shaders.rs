use maths::vector::vector3::Vector3;

use crate::graphics::{colour::Colour, interpolate::Interpolate, shapes_3d::point::Point};

pub trait Shaders {
	/// Output of vertex shader
	type VsOut: Interpolate;
	type Pixel;
	fn vertex(&self, index: usize, vertex: Point, normal: Vector3<f64>) -> Self::VsOut;

	fn fragment(&self, data: Self::VsOut) -> Self::Pixel;
}
