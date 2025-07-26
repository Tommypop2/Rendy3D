use crate::graphics::shapes_3d::triangle::Triangle3D;

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
	pub fn new(triangles: Vec<Triangle3D>) -> Self {
		Self { triangles }
	}
	pub fn dimensions(&self) {
		todo!();
		let first = self.triangles[0].clone();
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
