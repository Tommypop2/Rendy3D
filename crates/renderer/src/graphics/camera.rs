use crate::maths::matrices::matrix4::Matrix4;

use crate::graphics::viewport::Viewport;

pub struct Camera {
	/// Viewport which displays the camera view
	pub viewport: Viewport,
	/// Projection Matrix: converts from view space to the camera's image plane
	pub projection: Matrix4<f64>,
	/// Camera transformation
	pub transformation: Matrix4<f64>,
}
impl Camera {
	pub fn new(viewport: Viewport, projection: Matrix4<f64>) -> Self {
		Self {
			viewport,
			projection,
			transformation: Matrix4::identity(),
		}
	}
	/// Returns the view matrix
	///
	/// This is the matrix that transforms objects from world space to view space
	pub fn view(&self) -> Matrix4<f64> {
		self.transformation.reverse_rotation_translation()
	}
	/// Adds a transformation to this camera instance
	pub fn with_transformation(mut self, transformation: Matrix4<f64>) -> Self {
		self.transformation = transformation;
		self
	}
	pub fn direction() {
		todo!()
	}
}

#[cfg(test)]
mod test {}
