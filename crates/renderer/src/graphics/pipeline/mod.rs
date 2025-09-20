pub mod back_face_culling;

use crate::graphics::{
	interpolate::Interpolate, pipeline::back_face_culling::BackFaceCulling,
	shapes_2d::point::AbsoluteScreenCoordinate, shapes_3d::point::Point,
};

/// Render pipeline trait
///
/// This is where the vertex and fragment shaders are implemented. The back-face culling strategy is also selected here.
pub trait Pipeline {
	/// Output of vertex shader
	type VsOut: Interpolate;
	/// Type of fragments emitted by fragment shader
	type Fragment;
	/// Input vertex type
	type Vertex;

	/// Vertex shader
	fn vertex(&self, index: usize, vertex: Self::Vertex) -> (Point, Self::VsOut);

	/// Fragment shader
	fn fragment(&self, position: AbsoluteScreenCoordinate, data: Self::VsOut) -> Self::Fragment;

	/// Back-face Culling Mode
	fn backface_culling() -> BackFaceCulling {
		BackFaceCulling::None
	}
}
