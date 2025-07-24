use crate::graphics::{shapes_2d::triangle, shapes_3d::triangle::Triangle3D};

pub struct Mesh {
	/// Triangles that make up the shape
	triangles: Vec<Triangle3D>,
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
	pub fn dimensions(&self) {
		let first = self.triangles[0].clone();
		// let initial = Dimensions {
		// 	min_x: first
		// }
		self.triangles
			.iter()
			.fold((0.0, 0.0, 0.0), |mut dimensions, element| {
				let (width_x, width_y, width_z) = dimensions;

				dimensions
			});
	}
}
