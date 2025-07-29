use maths::matrices::matrix4::Matrix4;

use crate::graphics::{
	draw::Draw, screen::Screen, shapes_3d::triangle::Triangle3D, viewport::Viewport,
};

pub struct Camera {
	viewport: Viewport,
	perspective: Matrix4<f64>,
	transformation: Matrix4<f64>,
}
impl Camera {
	pub fn new(viewport: Viewport, perspective: Matrix4<f64>) -> Self {
		Self {
			viewport,
			perspective,
			transformation: Matrix4::unit(),
		}
	}

	pub fn render<T: Draw>(&mut self, screen: &mut Screen, triangles: &[Triangle3D]) {
		
	}
}
