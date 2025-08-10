use std::{
	collections::HashMap,
	fmt::Write,
	path::PathBuf,
	time::{Duration, Instant, SystemTime},
};

use argh::FromArgs;
use hsv::hsv_to_rgb;
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
		shaders::shaders::Shaders,
		shapes_2d::{bounding_area::BoundingArea2D, point::AbsoluteScreenCoordinate},
		target::Target,
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
fn draw_char(
	font: &fontdue::Font,
	offset: AbsoluteScreenCoordinate,
	frame_buffer: &mut [[Colour; WIDTH as usize]],
	ch: char,
	size: f32,
) -> fontdue::Metrics {
	let (metrics, bitmap) = font.rasterize(ch, size);
	if metrics.width == 0 {
		return metrics;
	}
	let rows = bitmap.chunks(metrics.width);
	for (row_index, row) in rows.into_iter().enumerate() {
		for (j, pixel) in frame_buffer[row_index + offset.y][(offset.x)..(offset.x + row.len())]
			.iter_mut()
			.enumerate()
		{
			*pixel = Colour::new(255, 255, 255, row[j])
		}
	}
	metrics
}
fn draw_text(
	font: &fontdue::Font,
	mut offset: AbsoluteScreenCoordinate,
	frame_buffer: &mut [[Colour; WIDTH as usize]],
	text: &str,
	size: f32,
) {
	for ch in text.chars() {
		offset.x += draw_char(font, offset, frame_buffer, ch, size).advance_width as usize
	}
}
struct FrameTimeCounter {
	time_of_last_frame: Instant,
}
impl FrameTimeCounter {
	pub fn new() -> Self {
		Self {
			time_of_last_frame: Instant::now(),
		}
	}
	/// Must be called on every frame
	pub fn frame_time(&mut self) -> Duration {
		let now = Instant::now();
		let frame_time = now - self.time_of_last_frame;
		self.time_of_last_frame = now;
		frame_time
	}
	pub fn fps(frame_time: Duration) -> f32 {
		1.0 / frame_time.as_secs_f32()
	}
}
#[derive(FromArgs)]
/// View STL files
struct Args {
	#[argh(positional)]
	file: PathBuf,
}
fn main() -> Result<(), Error> {
	let args: Args = argh::from_env();
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
	pixels.enable_vsync(false);
	let viewport =
		Viewport::new(BoundingArea2D::new(0, WIDTH as usize, 0, HEIGHT as usize)).unwrap();
	let perspective_matrix = perspective_matrix(1.0, 1.0, -20.0, 1.0);
	let camera = Camera::new(viewport, perspective_matrix.clone())
		.with_transformation(Matrix4::translation(Vector3::new(0.0, 0.0, 1.0)));
	let f1_car = Mesh::new(load_file(args.file));
	let mut scene = World::new(vec![camera], vec![Object::new(f1_car, Matrix4::identity())]);
	let mut control = FirstPersonControl::new(0.001);
	let mut z_buffer = vec![f32::NEG_INFINITY; { WIDTH * HEIGHT } as usize];
	let font = include_bytes!("../../Helvetica.ttf") as &[u8];
	let font = fontdue::Font::from_bytes(font, fontdue::FontSettings::default()).unwrap();
	let mut fps_counter = FrameTimeCounter::new();
	let mut fps_buffer = String::with_capacity(10);
	let res = event_loop.run(|event, elwt| {
		control.handle_event(&event, &mut scene.cameras[0]);
		if let Event::WindowEvent {
			event: WindowEvent::RedrawRequested,
			..
		} = event
		{
			let dt = fps_counter.frame_time();
			let fps = FrameTimeCounter::fps(dt);
			let mut screen = Screen::new(frame_pixels(pixels.frame_mut()), &mut z_buffer);
			screen.clear(Colour::BLACK);
			scene.draw(&mut screen);
			fps_buffer.clear();
			write!(&mut fps_buffer, "FPS: {fps:.0}").unwrap();
			draw_text(
				&font,
				AbsoluteScreenCoordinate::new(20, 20, 0.0),
				frame_pixels(pixels.frame_mut()),
				&fps_buffer,
				25.0,
			);
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
#[derive(Clone)]
struct CoolShaders {
	light_direction: Vector3<f64>,
}
impl Shaders for CoolShaders {
	type Pixel = Colour;
	type VsOut = Colour;
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
	fn fragment(&self, pos: AbsoluteScreenCoordinate, data: Self::VsOut) -> Self::Pixel {
		let z_normalised = pos.z / (56.241528 * 2.0) + 0.5;
		let (r, g, b) = hsv_to_rgb((pos.z * 360.0).clamp(0.0, 360.0) as f64 * 0.75, 1.0, 1.0);
		Colour::new(r, g, b, 255)
	}
}
impl World {
	fn new(cameras: Vec<Camera>, objects: Vec<Object>) -> Self {
		Self { objects, cameras }
	}

	fn draw(&mut self, screen: &mut Screen) {
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
					// &mut VertexShader::new(
					// 	Vector3::new(0.0, 0.0, 1.0),
					// 	|data, index, vertex, normal| {
					// 		let intensity = normal.dot_with(data);
					// 		let val = (255.0 * intensity) as u8;
					// 		Colour::new(val, val, val, 0xff)
					// 	},
					// ),
				);
			}
		}
	}
}
