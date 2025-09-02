pub mod back_face_culling;

use crate::graphics::{
	interpolate::Interpolate, pipeline::back_face_culling::BackFaceCulling,
	shapes_2d::point::AbsoluteScreenCoordinate, shapes_3d::point::Point,
};

pub trait Pipeline {
	/// Output of vertex shader
	type VsOut: Interpolate;
	type Fragment;
	type Vertex;

	/// Vertex shader
	fn vertex(&self, index: usize, vertex: Self::Vertex) -> (Point, Self::VsOut);

	/// Fragment shader
	fn fragment(&self, position: AbsoluteScreenCoordinate, data: Self::VsOut) -> Self::Fragment;

	/// How triangles should be culled
	fn backface_culling() -> BackFaceCulling {
		BackFaceCulling::None
	}
}
