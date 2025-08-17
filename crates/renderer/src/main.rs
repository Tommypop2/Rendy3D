use std::time::{Instant, SystemTime};

use error_iter::ErrorIter as _;
use log::error;
use maths::matrices::matrix4::Matrix4;
use maths::vector::vector2::Vector2;
use maths::vector::vector3::Vector3;
use pixels::{Error, Pixels, SurfaceTexture};
use rendy3d::graphics::camera::Camera;
use rendy3d::graphics::colour::Colour;
use rendy3d::graphics::draw::Draw;
use rendy3d::graphics::interpolate::Interpolate;
use rendy3d::graphics::screen::{Screen, frame_pixels};
use rendy3d::graphics::shaders::shaders::Pipeline;
use rendy3d::graphics::shapes_2d::bounding_area::BoundingArea2D;
use rendy3d::graphics::shapes_2d::point::AbsoluteScreenCoordinate;
use rendy3d::graphics::shapes_2d::triangle::Triangle;
use rendy3d::graphics::target::Target;
use rendy3d::graphics::texture::Texture;
use rendy3d::graphics::viewport::Viewport;
use rendy3d::loaders::obj::{Mesh, TexturedVertex, load_obj};
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::KeyCode;
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;
struct World {
	pub cameras: Vec<Camera>,
	pub objects: Vec<Mesh>,
}

fn main() -> Result<(), Error> {
	env_logger::init();
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

	let mut pixels = {
		let window_size = window.inner_size();
		let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
		Pixels::new(WIDTH, HEIGHT, surface_texture)?
	};
	// let pers_mat = perspective_matrix(1.0, 1.0, -20.0, 1.0);
	let pers_mat = Matrix4::identity();

	let viewport =
		Viewport::new(BoundingArea2D::new(0, WIDTH as usize, 0, HEIGHT as usize)).unwrap();
	let main_camera = Camera::new(viewport, pers_mat.clone())
		.with_transformation(Matrix4::translation(Vector3::new(0.0, 0.0, 1.0)));
	// let viewport2 = Viewport::new(BoundingArea2D::new(
	// 	(WIDTH / 2) as usize,
	// 	WIDTH as usize,
	// 	0,
	// 	HEIGHT as usize,
	// ))
	// .unwrap();
	// let second_camera = Camera::new(viewport2, pers_mat.clone());
	// let object = Mesh::new(vec![
	// 	Triangle3D::new(
	// 		Point::new(0.0, 0.0, 0.0),
	// 		Point::new(0.5, 0.0, 0.0),
	// 		Point::new(0.1, 0.4, 0.0),
	// 	),
	// 	Triangle3D::new(
	// 		Point::new(0.1, 0.4, 0.0),
	// 		Point::new(0.5, 0.0, 0.0),
	// 		Point::new(0.8, 0.4, 0.0),
	// 	),
	// ]);
	let object = load_obj("./barrel_03_4k.obj").unwrap();
	// let object = Mesh::new(load_file("./F1_RB16B.stl"));
	// let guinea_pig = Mesh::new(load_file("./GatlingGuineaPig.stl"));
	let mut scene = World::new(vec![main_camera], vec![object]);
	// let mut scene2 = World::new(vec![second_camera], vec![guinea_pig]);
	let mut frame_num: usize = 0;
	let mut sum: u128 = 0;
	let mut shaders = Test {
		light_direction: Vector3::new(0.0, 0.0, 1.0),
		texture: Texture::from_path("barrel_03_diff_4k.jpg"),
	};
	// let pers_mat = Matrix4::unit();
	let mut z_buffer = vec![f32::NEG_INFINITY; { WIDTH * HEIGHT } as usize];
	let res = event_loop.run(|event, elwt| {
		let mut screen = Screen::new(
			frame_pixels(pixels.frame_mut()),
			&mut z_buffer,
			WIDTH as usize,
			HEIGHT as usize,
		);
		if let Event::WindowEvent {
			event: WindowEvent::RedrawRequested,
			..
		} = event
		{
			// Clear buffer
			let start = Instant::now();
			screen.clear(Colour::BLACK);
			scene.draw(&mut screen, &mut shaders);
			// scene2.draw(&mut screen);
			let time_taken = start.elapsed();
			frame_num += 1;
			sum += time_taken.as_micros();
			if frame_num % 1000 == 0 {
				//
				let mean = sum as f64 / frame_num as f64;
				frame_num = 0;
				sum = 0;
				println!(
					"Mean draw time taken over most recent 1000 frames is {mean} microseconds"
				);
				println!("This is {} FPS", 1E6 / mean)
			}

			if let Err(err) = pixels.render() {
				log_error("pixels.render", err);
				elwt.exit();
				return;
			}
		}

		if input.update(&event) {
			if input.key_pressed(KeyCode::Escape) || input.close_requested() {
				elwt.exit();
				return;
			}

			if let Some(size) = input.window_resized() {
				if let Err(err) = pixels.resize_surface(size.width, size.height) {
					log_error("pixels.resize_surface", err);
					elwt.exit();
					return;
				}
			}

			scene.update();
			window.request_redraw();
		}
	});
	res.map_err(|e| Error::UserDefined(Box::new(e)))
}

fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
	error!("{method_name}() failed: {err}");
	for source in err.sources().skip(1) {
		error!("  Caused by: {source}");
	}
}
struct Test {
	light_direction: Vector3<f64>,
	texture: Texture,
}
impl Pipeline for Test {
	type VsOut = Vector2<f64>;
	type Vertex = TexturedVertex;
	type Fragment = Colour;

	fn vertex(&self, index: usize, vertex: Self::Vertex) -> Self::VsOut {
		// let intensity = vertex.normal.dot_with(&self.light_direction);
		// let val = (255.0 * intensity) as u8;
		// Colour::new(val, val, val, 0xff)
		vertex.texture
		// let res = index % 3;
		// match res {
		// 	0 => Colour::RED,
		// 	1 => Colour::GREEN,
		// 	2 => Colour::BLUE,
		// 	_ => unreachable!(),
		// }
	}

	fn fragment(&self, pos: AbsoluteScreenCoordinate, data: Self::VsOut) -> Self::Fragment {
		// let intensity = data.dot_with(&self.light_direction);
		// let val = (255.0 * intensity) as u8;
		// Colour::new(val, val, val, 0xff)
		self.texture.get_pixel(data.x as f32, data.y as f32)
		// data
	}
}
impl World {
	fn new(cameras: Vec<Camera>, objects: Vec<Mesh>) -> Self {
		Self { objects, cameras }
	}

	fn update(&mut self) {}

	fn draw<U: Interpolate, T: Pipeline<VsOut = U, Fragment = Colour, Vertex = TexturedVertex>>(
		&mut self,
		screen: &mut Screen,
		shaders: &mut T,
	) {
		let x: std::time::Duration = SystemTime::now()
			.duration_since(SystemTime::UNIX_EPOCH)
			.unwrap();
		let base_transform = Matrix4::rotation_z(x.as_secs_f64())
			* Matrix4::rotation_y(x.as_secs_f64())
			* Matrix4::rotation_x(x.as_secs_f64());
		// let base_transform = Matrix4::identity();
		for object in &self.objects {
			for camera in &mut self.cameras {
				let transform = camera.view()
					* Matrix4::scale_x(
						camera.viewport.area.height() as f64 / camera.viewport.area.width() as f64,
					) * base_transform.clone();

				// render_mesh(
				// 	&mut camera.viewport.target(screen),
				// 	&object.mesh.triangles,
				// 	transform,
				// 	camera.projection.clone(),
				// 	&mut Test,
				// );
				let target = &mut camera.viewport.target(screen);

				for chunk in object.indices.chunks_exact(3) {
					let i1 = chunk[0];
					let i2 = chunk[1];
					let i3 = chunk[2];
					let v1 = object.vertices[i1 as usize];
					let v2 = object.vertices[i2 as usize];
					let v3 = object.vertices[i3 as usize];
					let triangle = Triangle::new(v1, v2, v3);
					// let triangle = Triangle3D::new(
					// 	Point::from_vector(v1.position.map_components(|x| x as f64)),
					// 	Point::from_vector(v2.position.map_components(|x| x as f64)),
					// 	Point::from_vector(v3.position.map_components(|x| x as f64)),
					// );
					// Render triangle
					let transformed = triangle.apply(transform.clone());
					// let n = transformed.normal().normalized();
					// let intensity = n.dot_with(&camera_dir);
					// Back-face culling :)
					// if intensity < 0.0 {
					// continue;
					// }
					let projected = transformed.apply(camera.projection.clone());
					// Triangle2D::new(vertex1, vertex2, vertex3).draw(target, shaders);
					Triangle::new(
						(
							projected
								.vertex1
								.position
								.to_pixel_coordinate(target.area()),
							shaders.vertex(0, v1),
						),
						(
							projected
								.vertex2
								.position
								.to_pixel_coordinate(target.area()),
							shaders.vertex(1, v2),
						),
						(
							projected
								.vertex3
								.position
								.to_pixel_coordinate(target.area()),
							shaders.vertex(2, v3),
						),
					)
					.draw(target, shaders);
					// transformed
					// 	.apply(camera.projection.clone())
					// 	.to_triangle_2d(target, shaders, n)
					// 	.draw(target, shaders);
				}
				// for (i, triangle) in mesh.iter().enumerate() {
				// 	let transformed = triangle.clone().apply(transform.clone());
				// 	let n = transformed.normal().normalized();
				// 	let intensity = n.dot_with(&camera_dir);
				// 	// Back-face culling :)
				// 	if intensity < 0.0 {
				// 		continue;
				// 	}
				// 	transformed
				// 		.apply(camera.projection.clone())
				// 		.to_triangle_2d(target, shaders, n)
				// 		.draw(target, shaders);
				// }
			}
		}
	}
}
