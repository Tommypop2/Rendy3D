use std::{collections::HashMap, time::SystemTime};

use pixels::{Error, Pixels, SurfaceTexture};
use rendy3d::{
	HEIGHT, WIDTH,
	graphics::{
		camera::Camera,
		colour::Colour,
		mesh::{Mesh, render_mesh},
		object::Object,
		perspective::perspective_matrix,
		screen::{Screen, frame_pixels},
		shaders::vertex::VertexShader,
		shapes_2d::bounding_area::BoundingArea2D,
		viewport::Viewport,
	},
	loaders::stl::load_file,
	maths::{matrices::matrix4::Matrix4, vector::vector3::Vector3},
};
use winit::{
	dpi::LogicalSize,
	event::{DeviceEvent, ElementState, Event, MouseButton, RawKeyEvent, WindowEvent},
	event_loop::EventLoop,
	keyboard::{KeyCode, PhysicalKey},
	platform::scancode::PhysicalKeyExtScancode,
	window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

struct World {
	pub cameras: Vec<Camera>,
	pub objects: Vec<Object>,
}
struct FirstPersonControl {
	click_pressed: bool,
	speed: f64,
	keys_pressed: HashMap<KeyCode, bool>,
}
impl FirstPersonControl {
	pub fn new(speed: f64) -> Self {
		Self {
			speed,
			click_pressed: false,
			keys_pressed: HashMap::new(),
		}
	}
	pub fn handle_event(&mut self, event: &Event<()>, camera: &mut Camera) {
		for (code, pressed) in self.keys_pressed.iter() {
			if !*pressed {
				continue;
			}
			let transform = match code {
				KeyCode::KeyW => Matrix4::translation(Vector3::new(0.0, 0.0, -self.speed)),
				KeyCode::KeyS => Matrix4::translation(Vector3::new(0.0, 0.0, self.speed)),
				KeyCode::KeyA => Matrix4::translation(Vector3::new(self.speed, 0.0, 0.0)),
				KeyCode::KeyD => Matrix4::translation(Vector3::new(-self.speed, 0.0, 0.0)),
				KeyCode::Space => Matrix4::translation(Vector3::new(0.0, -self.speed, 0.0)),
				KeyCode::ShiftLeft | KeyCode::ShiftRight => {
					Matrix4::translation(Vector3::new(0.0, self.speed, 0.0))
				}
				_ => {
					continue;
				}
			};
			camera.transformation = camera.transformation.clone() * transform;
		}
		if let Event::WindowEvent {
			window_id: _,
			event:
				WindowEvent::MouseInput {
					device_id: _,
					state,
					button: MouseButton::Left,
				},
		} = event
		{
			self.click_pressed = match state {
				ElementState::Pressed => true,
				ElementState::Released => false,
			}
		}
		if let Event::DeviceEvent {
			event,
			device_id: _,
		} = event
		{
			match event {
				DeviceEvent::MouseMotion { delta } => {
					if !self.click_pressed {
						return;
					}
					let dx = delta.0 * self.speed;
					let dy = delta.1 * self.speed;
					camera.transformation = camera.transformation.clone()
						* Matrix4::rotation_y(dy)
						* Matrix4::rotation_z(dx)
				}
				DeviceEvent::Key(RawKeyEvent {
					physical_key: PhysicalKey::Code(code),
					state,
				}) => {
					self.keys_pressed.insert(
						*code,
						match state {
							ElementState::Pressed => true,
							ElementState::Released => false,
						},
					);
				}
				_ => {}
			}
		}
	}
}
fn main() -> Result<(), Error> {
	let event_loop = EventLoop::new().unwrap();
	let mut input = WinitInputHelper::new();
	let window = {
		let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
		WindowBuilder::new()
			.with_title("Viewy3D")
			.with_inner_size(size)
			.with_min_inner_size(size)
			.build(&event_loop)
			.unwrap()
	};
	let mut pixels = {
		let window_size = window.inner_size();
		let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
		Pixels::new(WIDTH, HEIGHT, surface_texture)?
	};

	let viewport =
		Viewport::new(BoundingArea2D::new(0, WIDTH as usize, 0, HEIGHT as usize)).unwrap();
	let perspective_matrix = perspective_matrix(1.0, 1.0, -20.0, 1.0);
	let camera = Camera::new(viewport, perspective_matrix.clone())
		.with_transformation(Matrix4::translation(Vector3::new(0.0, 0.0, 1.0)));
	let f1_car = Mesh::new(load_file("../F1_RB16B.stl"));
	let mut scene = World::new(vec![camera], vec![Object::new(f1_car, Matrix4::identity())]);
	let mut control = FirstPersonControl::new(0.001);
	let mut z_buffer = vec![f32::NEG_INFINITY; { WIDTH * HEIGHT } as usize];
	let res = event_loop.run(|event, elwt| {
		let mut screen = Screen::new(frame_pixels(pixels.frame_mut()), &mut z_buffer);
		control.handle_event(&event, &mut scene.cameras[0]);
		if let Event::WindowEvent {
			event: WindowEvent::RedrawRequested,
			..
		} = event
		{
			screen.clear(Colour::BLACK);
			scene.draw(&mut screen);
			pixels.render().unwrap();
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
		let x: std::time::Duration = SystemTime::now()
			.duration_since(SystemTime::UNIX_EPOCH)
			.unwrap();
		let base_transform = Matrix4::scale(0.01)
			* Matrix4::rotation_z(x.as_secs_f64())
			* Matrix4::rotation_y(x.as_secs_f64())
			* Matrix4::rotation_x(x.as_secs_f64());

		for object in &self.objects {
			for camera in &mut self.cameras {
				let transform = Matrix4::scale_x(
					camera.viewport.area.height() as f64 / camera.viewport.area.width() as f64,
				) * camera.view()
					* base_transform.clone();

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
