use std::{fmt::Write, num::NonZeroU32, path::PathBuf, rc::Rc};

use argh::FromArgs;
use pixels::Error;
use rendy3d::{
	graphics::{
		camera::Camera,
		colour::Colour,
		mesh::Mesh,
		object::Object,
		screen::Screen,
		shapes_2d::{bounding_area::BoundingArea2D, point::AbsoluteScreenCoordinate},
		shapes_3d::{point::Point, triangle::Triangle3D},
		target::Target,
		viewport::Viewport,
	},
	loaders::stl::load_file,
	maths::{matrices::matrix4::Matrix4, vector::vector3::Vector3},
};
use winit::{
	application::ApplicationHandler,
	dpi::PhysicalSize,
	event::WindowEvent,
	event_loop::{ActiveEventLoop, EventLoop},
	window::{Window, WindowId},
};
use winit_input_helper::WinitInputHelper;

use crate::{
	control::FirstPersonControl, frame_time_counter::FrameTimeCounter, text::draw_text,
	world::World,
};
pub mod control;
pub mod frame_time_counter;
pub mod text;
pub mod world;

#[derive(FromArgs)]
/// View STL files
struct Args {
	#[argh(positional)]
	file: PathBuf,
}

struct App {
	window: Option<Rc<Window>>,
	control: FirstPersonControl,
	frame_time_counter: FrameTimeCounter,
	scene: World,
	font: fontdue::Font,
	z_buffer: Vec<f32>,
	surface: Option<softbuffer::Surface<Rc<Window>, Rc<Window>>>,
}
impl ApplicationHandler for App {
	fn resumed(&mut self, event_loop: &ActiveEventLoop) {
		let window = Rc::new(
			event_loop
				.create_window(
					Window::default_attributes().with_inner_size(PhysicalSize::new(1280, 720)),
				)
				.unwrap(),
		);
		let context = softbuffer::Context::new(window.clone()).unwrap();
		let surface: softbuffer::Surface<Rc<Window>, Rc<Window>> =
			softbuffer::Surface::new(&context, window.clone()).unwrap();
		self.window = Some(window);
		self.surface = Some(surface)
	}
	fn device_event(
		&mut self,
		event_loop: &ActiveEventLoop,
		device_id: winit::event::DeviceId,
		event: winit::event::DeviceEvent,
	) {
		self.control
			.handle_device_event(&event, &mut self.scene.cameras[0]);
	}
	fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
		self.control.handle_window_event(&event);
		match event {
			WindowEvent::CloseRequested => {
				println!("The close button was pressed; stopping");
				event_loop.exit();
			}
			WindowEvent::Resized(size) => {
				let width = size.width;
				let height = size.height;
				println!("{width}, {height}");
				// Ensure that Z buffer has enough space allocated
				self.z_buffer = vec![f32::NEG_INFINITY; { width * height } as usize];
				self.surface
					.as_mut()
					.unwrap()
					.resize(
						NonZeroU32::new(width).unwrap(),
						NonZeroU32::new(height).unwrap(),
					)
					.unwrap();
				// Adjust viewport size
				self.scene.cameras[0].viewport.set_area(BoundingArea2D::new(
					0,
					width as usize,
					0,
					height as usize,
				));
			}
			WindowEvent::RedrawRequested => {
				self.control.step(&mut self.scene.cameras[0]);
				// Redraw the application.
				//
				// It's preferable for applications that do not render continuously to render in
				// this event rather than in AboutToWait, since rendering in here allows
				// the program to gracefully handle redraws requested by the OS.
				let PhysicalSize { width, height } = self.window.as_ref().unwrap().inner_size();
				let mut fps_buffer = String::with_capacity(10);
				// Draw.
				let dt = self.frame_time_counter.frame_time();
				let fps = FrameTimeCounter::fps(dt);
				let mut buffer = self.surface.as_mut().unwrap().buffer_mut().unwrap();
				let pixels = &mut *buffer as *mut [u32];
				let frame_buffer = unsafe { &mut *(pixels as *mut [Colour]) };
				let mut screen = Screen::new(
					frame_buffer,
					&mut self.z_buffer,
					width as usize,
					height as usize,
				);
				screen.clear(Colour::BLACK);
				self.scene.draw(&mut screen);
				fps_buffer.clear();
				write!(&mut fps_buffer, "FPS: {fps:.0}").unwrap();
				draw_text(
					&self.font,
					AbsoluteScreenCoordinate::new(20, 20, 0.0),
					frame_buffer,
					&fps_buffer,
					25.0,
					width as usize,
				);
				buffer.present().unwrap();
				// Queue a RedrawRequested event.
				//
				// You only need to call this if you've determined that you need to redraw in
				// applications which do not always need to. Applications that redraw continuously
				// can render here instead.
				self.window.as_ref().unwrap().request_redraw();
			}
			_ => (),
		}
	}
}
impl App {
	pub fn new(scene: World, font: fontdue::Font) -> Self {
		Self {
			window: None,
			control: FirstPersonControl::new(0.001),
			frame_time_counter: FrameTimeCounter::new(),
			scene,
			font,
			z_buffer: Vec::new(),
			surface: None,
		}
	}
}
fn main() -> Result<(), Error> {
	let args: Args = argh::from_env();
	let event_loop = EventLoop::new().unwrap();
	let input = WinitInputHelper::new();
	let viewport = Viewport::new(BoundingArea2D::new(0, 0_usize, 0, 0_usize)).unwrap();
	let perspective_matrix = Matrix4::new_perspective(1.0, 1.0, -20.0, 1.0);
	let camera = Camera::new(viewport, perspective_matrix.clone())
		.with_transformation(Matrix4::translation(Vector3::new(0.0, 0.0, 1.0)));
	let f1_car = Mesh::new(load_file(args.file));
	let object = Mesh::new(vec![Triangle3D::new(
		Point::new(0.0, 0.0, 0.0),
		Point::new(0.5, 0.0, 0.0),
		Point::new(0.1, 0.4, 0.0),
	)]);
	// let object = Mesh::new(load_file("./F1_RB16B.stl"));
	let guinea_pig = Mesh::new(load_file("../GatlingGuineaPig.stl"));
	let scene = World::new(
		vec![camera],
		vec![Object::new(guinea_pig, Matrix4::identity())],
	);
	// let mut scene = World::new(vec![camera], vec![Object::new(f1_car, Matrix4::identity())]);
	let font = include_bytes!("../../Helvetica.ttf") as &[u8];
	let font = fontdue::Font::from_bytes(font, fontdue::FontSettings::default()).unwrap();

	// let res = event_loop.run(|event, elwt| {
	// 	control.handle_event(&event, &mut scene.cameras[0]);
	// 	if let Event::WindowEvent {
	// 		event: WindowEvent::Resized(size),
	// 		..
	// 	} = event
	// 	{}
	// 	if let Event::WindowEvent {
	// 		event: WindowEvent::RedrawRequested,
	// 		..
	// 	} = event
	// 	{}

	// 	if input.update(&event) {
	// 		if input.key_pressed(KeyCode::Escape) || input.close_requested() {
	// 			elwt.exit();
	// 			return;
	// 		}

	// 		window.request_redraw();
	// 	}
	// });
	let mut app = App::new(scene, font);
	let res = event_loop.run_app(&mut app);
	res.map_err(|e| Error::UserDefined(Box::new(e)))
}
