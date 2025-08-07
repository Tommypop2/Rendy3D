use std::time::SystemTime;

use pixels::{Error, Pixels, SurfaceTexture};
use rendy3d::{
	HEIGHT, WIDTH,
	graphics::{
		camera::Camera,
		colour::Colour,
		mesh::{Mesh, render_mesh},
		object::Object,
		perspective::perspective_matrix,
		screen::Screen,
		shaders::vertex::VertexShader,
		shapes_2d::bounding_area::BoundingArea2D,
		viewport::Viewport,
	},
	loaders::stl::load_file,
	maths::{matrices::matrix4::Matrix4, vector::vector3::Vector3},
};
use winit::{
	dpi::LogicalSize,
	event::{DeviceEvent, Event, WindowEvent},
	event_loop::EventLoop,
	keyboard::KeyCode,
	window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

struct World {
	pub cameras: Vec<Camera>,
	pub objects: Vec<Object>,
}
fn main() -> Result<(), Error> {
	let event_loop = EventLoop::new().unwrap();
	let mut input = WinitInputHelper::new();
	let window = {
		let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
		WindowBuilder::new()
			.with_title("Rasterizer")
			.with_inner_size(size)
			.with_min_inner_size(size)
			.build(&event_loop)
			.unwrap()
	};
	let pixels = {
		let window_size = window.inner_size();
		let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
		Pixels::new(WIDTH, HEIGHT, surface_texture)?
	};
	let mut screen = Screen::new(pixels);
	let viewport =
		Viewport::new(BoundingArea2D::new(0, WIDTH as usize, 0, HEIGHT as usize)).unwrap();
	let perspective_matrix = perspective_matrix(1.0, 1.0, -20.0, 1.0);
	let main_camera = Camera::new(viewport, perspective_matrix.clone())
		.with_transformation(Matrix4::translation(Vector3::new(0.0, 0.0, 1.0)));
	let f1_car = Mesh::new(load_file("../F1_RB16B.stl"));
	let mut scene = World::new(
		vec![main_camera],
		vec![Object::new(f1_car, Matrix4::identity())],
	);
	let res = event_loop.run(|event, elwt| {
		if let Event::WindowEvent {
			event: WindowEvent::RedrawRequested,
			..
		} = event
		{
			screen.clear(Colour::BLACK);
			scene.draw(&mut screen);
			screen.pixels.render().unwrap();
		}

		if input.update(&event) {
			if input.key_pressed(KeyCode::Escape) || input.close_requested() {
				elwt.exit();
				return;
			}

			window.request_redraw();
		}
	});
	res.map_err(|e| Error::UserDefined(Box::new(e)))
}

impl World {
	fn new(cameras: Vec<Camera>, objects: Vec<Object>) -> Self {
		Self { objects, cameras }
	}

	fn draw(&mut self, screen: &mut Screen) {
		let _x: std::time::Duration = SystemTime::now()
			.duration_since(SystemTime::UNIX_EPOCH)
			.unwrap();
		// * Matrix4::rotation_z(x.as_secs_f64())
		// * Matrix4::rotation_y(x.as_secs_f64())
		// * Matrix4::rotation_x(x.as_secs_f64())
		for object in &self.objects {
			for camera in &mut self.cameras {
				let transform = camera.view()
					* Matrix4::scale_x(
						camera.viewport.area.height() as f64 / camera.viewport.area.width() as f64,
					) * Matrix4::scale(0.01);

				render_mesh(
					&mut camera.viewport,
					screen,
					&object.mesh.triangles,
					transform,
					camera.projection.clone(),
					&mut VertexShader::new(
						Vector3::new(0.0, 0.0, 1.0),
						|data, index, vertex, normal| {
							let intensity = normal.dot_with(data);
							let val = (255.0 * intensity) as u8;
							Colour::new(val, val, val, 0xff)
						},
					),
				);
			}
		}
	}
}
