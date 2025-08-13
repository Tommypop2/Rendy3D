use std::{fmt::Write, path::PathBuf};

use argh::FromArgs;
use pixels::{Error, Pixels, SurfaceTexture};
use rendy3d::{
	graphics::{
		camera::Camera,
		colour::Colour,
		mesh::Mesh,
		object::Object,
		perspective::perspective_matrix,
		screen::{Screen, frame_pixels},
		shapes_2d::{bounding_area::BoundingArea2D, point::AbsoluteScreenCoordinate},
		target::Target,
		viewport::Viewport,
	},
	loaders::stl::load_file,
	maths::{matrices::matrix4::Matrix4, vector::vector3::Vector3},
};
use winit::{
	dpi::LogicalSize,
	event::{Event, WindowEvent},
	event_loop::EventLoop,
	keyboard::KeyCode,
	window::WindowBuilder,
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

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;

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
			let mut screen = Screen::new(
				frame_pixels(pixels.frame_mut()),
				&mut z_buffer,
				WIDTH as usize,
				HEIGHT as usize,
			);
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
				WIDTH as usize,
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
