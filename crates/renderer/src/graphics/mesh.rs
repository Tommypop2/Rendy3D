use maths::matrices::matrix4::Matrix4;

use crate::graphics::{
	draw::Draw,
	interpolate::Interpolate,
	pipeline::pipeline::Pipeline,
	shapes_3d::{point::Point, triangle::Triangle3D},
	target::Target,
};

pub struct Mesh {
	/// Triangles that make up the shape
	pub triangles: Vec<Triangle3D>,
}
#[derive(Default)]
pub struct Dimensions {
	min_x: f64,
	min_y: f64,
	min_z: f64,
	max_x: f64,
	max_y: f64,
	max_z: f64,
}
impl Mesh {
	pub fn new(triangles: Vec<Triangle3D>) -> Self {
		Self { triangles }
	}
	pub fn dimensions(&self) {
		todo!();
		// let first = self.triangles[0].clone();
		// let initial = Dimensions {
		// 	min_x: first
		// }
		self.triangles
			.iter()
			.fold((0.0, 0.0, 0.0), |dimensions, element| {
				let (width_x, width_y, width_z) = dimensions;

				dimensions
			});
	}
}
