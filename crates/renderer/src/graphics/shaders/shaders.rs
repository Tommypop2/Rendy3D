use crate::graphics::{interpolate::Interpolate, shapes_2d::point::AbsoluteScreenCoordinate};

pub trait Pipeline {
	/// Output of vertex shader
	type VsOut: Interpolate;
	type Fragment;
	type Vertex;

	fn vertex(&self, index: usize, vertex: Self::Vertex) -> Self::VsOut;

	fn fragment(&self, position: AbsoluteScreenCoordinate, data: Self::VsOut) -> Self::Fragment;
}
