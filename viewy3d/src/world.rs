use std::time::SystemTime;

use hsv::hsv_to_rgb;
use rendy3d::{
	graphics::{
		camera::Camera, colour::Colour, mesh::render_mesh, object::Object, screen::Screen,
		shaders::shaders::Shaders, shapes_2d::point::AbsoluteScreenCoordinate,
	},
	maths::{matrices::matrix4::Matrix4, vector::vector3::Vector3},
};

pub struct World {
	pub cameras: Vec<Camera>,
	pub objects: Vec<Object>,
}

impl World {
	pub fn new(cameras: Vec<Camera>, objects: Vec<Object>) -> Self {
		Self { objects, cameras }
	}

	pub fn draw(&mut self, screen: &mut Screen) {
		let x: std::time::Duration = SystemTime::now()
			.duration_since(SystemTime::UNIX_EPOCH)
			.unwrap();
		let base_transform = Matrix4::scale(0.01);
		// * Matrix4::rotation_z(x.as_secs_f64())
		// * Matrix4::rotation_y(x.as_secs_f64())
		// * Matrix4::rotation_x(x.as_secs_f64());

		for object in &self.objects {
			for camera in &mut self.cameras {
				let transform = Matrix4::scale_x(
					camera.viewport.area.height() as f64 / camera.viewport.area.width() as f64,
				) * camera.view()
					* base_transform.clone();

				render_mesh(
					&mut camera.viewport.target(screen),
					&object.mesh.triangles,
					transform,
					camera.projection.clone(),
					&mut CoolShaders {
						light_direction: Vector3::new(0.0, 0.0, 1.0),
					},
				);
			}
		}
	}
}
#[derive(Clone)]
struct CoolShaders {
	light_direction: Vector3<f64>,
}
impl Shaders for CoolShaders {
	type Fragment = Colour;
	type VsOut = Colour;
	type Vertex = Point;
	fn vertex(
		&self,
		index: usize,
		vertex: rendy3d::graphics::shapes_3d::point::Point,
		normal: Vector3<f64>,
	) -> Self::VsOut {
		let intensity = normal.dot_with(&self.light_direction);
		let val = (255.0 * intensity) as u8;
		Colour::new(val, val, val, 0xff)
	}
	fn fragment(&self, pos: AbsoluteScreenCoordinate, data: Self::VsOut) -> Self::Fragment {
		let z_normalised = pos.z / (56.241528 * 2.0) + 0.5;
		let (r, g, b) = hsv_to_rgb((pos.z * 360.0).clamp(0.0, 360.0) as f64 * 0.75, 1.0, 1.0);
		Colour::new(r, g, b, 255)
	}
}
