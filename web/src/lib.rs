use hsv::hsv_to_rgb;
use rendy3d::graphics::colour::Colour;
use rendy3d::graphics::mesh::render_mesh;
use rendy3d::graphics::perspective::perspective_matrix;
// Derived from softbuffer `winit` example
use log::debug;
use rendy3d::graphics::pipeline::pipeline::Pipeline;
use rendy3d::graphics::screen::Screen;
use rendy3d::graphics::shapes_2d::bounding_area::BoundingArea2D;
use rendy3d::graphics::shapes_2d::point::AbsoluteScreenCoordinate;
use rendy3d::graphics::shapes_2d::triangle::Triangle;
use rendy3d::graphics::shapes_3d::point::Point;
use rendy3d::graphics::target::Target;
use rendy3d::graphics::viewport::Viewport;
use rendy3d::maths::matrices::matrix4::Matrix4;
use rendy3d::maths::vector::vector3::Vector3;
#[cfg(target_arch = "wasm32")]
use std::cell::RefCell;
use std::marker::PhantomData;
use std::num::NonZeroU32;
use std::rc::Rc;
use std::time::SystemTime;
use web_sys::Performance;
use winit::application::ApplicationHandler;
use winit::event::{Event, KeyEvent, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::keyboard::{Key, NamedKey};
use winit::window::{Window, WindowAttributes, WindowId};

/// Run a Winit application.
#[allow(unused_mut)]
pub(crate) fn run_app(event_loop: EventLoop<()>, mut app: impl ApplicationHandler<()> + 'static) {
	#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
	event_loop.run_app(&mut app).unwrap();

	#[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
	winit::platform::web::EventLoopExtWebSys::spawn_app(event_loop, app);
}
#[cfg(target_arch = "wasm32")]
thread_local! {
	static WINDOW: RefCell<Option<HtmlCanvasElement>> = RefCell::new(None);
}
/// Create a window from a set of window attributes.
#[allow(dead_code)]
pub(crate) fn make_window(
	elwt: &ActiveEventLoop,
	f: impl FnOnce(WindowAttributes) -> WindowAttributes,
) -> Rc<Window> {
	let attributes = f(WindowAttributes::default());
	#[cfg(target_arch = "wasm32")]
	let attributes = {
		use winit::platform::web::WindowAttributesExtWebSys;
		winit::platform::web::WindowAttributesExtWebSys::with_canvas(attributes, WINDOW.take())
			.with_append(true)
	};
	let window = elwt.create_window(attributes).unwrap();
	Rc::new(window)
}

/// Easily constructable winit application.
pub(crate) struct WinitApp<T, S, Init, InitSurface, Handler> {
	/// Closure to initialize `state`.
	init: Init,

	/// Closure to initialize `surface_state`.
	init_surface: InitSurface,

	/// Closure to run on window events.
	event: Handler,

	/// Contained state.
	state: Option<T>,

	/// Contained surface state.
	surface_state: Option<S>,
}

/// Builder that makes it so we don't have to name `T`.
pub(crate) struct WinitAppBuilder<T, S, Init, InitSurface> {
	/// Closure to initialize `state`.
	init: Init,

	/// Closure to initialize `surface_state`.
	init_surface: InitSurface,

	/// Eat the type parameter.
	_marker: PhantomData<(Option<T>, Option<S>)>,
}

impl<T, S, Init, InitSurface> WinitAppBuilder<T, S, Init, InitSurface>
where
	Init: FnMut(&ActiveEventLoop) -> T,
	InitSurface: FnMut(&ActiveEventLoop, &mut T) -> S,
{
	/// Create with an "init" closure.
	pub(crate) fn with_init(init: Init, init_surface: InitSurface) -> Self {
		Self {
			init,
			init_surface,
			_marker: PhantomData,
		}
	}

	/// Build a new application.
	pub(crate) fn with_event_handler<F>(self, handler: F) -> WinitApp<T, S, Init, InitSurface, F>
	where
		F: FnMut(&mut T, Option<&mut S>, Event<()>, &ActiveEventLoop),
	{
		WinitApp::new(self.init, self.init_surface, handler)
	}
}

impl<T, S, Init, InitSurface, Handler> WinitApp<T, S, Init, InitSurface, Handler>
where
	Init: FnMut(&ActiveEventLoop) -> T,
	InitSurface: FnMut(&ActiveEventLoop, &mut T) -> S,
	Handler: FnMut(&mut T, Option<&mut S>, Event<()>, &ActiveEventLoop),
{
	/// Create a new application.
	pub(crate) fn new(init: Init, init_surface: InitSurface, event: Handler) -> Self {
		Self {
			init,
			init_surface,
			event,
			state: None,
			surface_state: None,
		}
	}
}

impl<T, S, Init, InitSurface, Handler> ApplicationHandler
	for WinitApp<T, S, Init, InitSurface, Handler>
where
	Init: FnMut(&ActiveEventLoop) -> T,
	InitSurface: FnMut(&ActiveEventLoop, &mut T) -> S,
	Handler: FnMut(&mut T, Option<&mut S>, Event<()>, &ActiveEventLoop),
{
	fn resumed(&mut self, el: &ActiveEventLoop) {
		debug_assert!(self.state.is_none());
		let mut state = (self.init)(el);
		self.surface_state = Some((self.init_surface)(el, &mut state));
		self.state = Some(state);
	}

	fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
		let surface_state = self.surface_state.take();
		debug_assert!(surface_state.is_some());
		drop(surface_state);
	}

	fn window_event(
		&mut self,
		event_loop: &ActiveEventLoop,
		window_id: WindowId,
		event: WindowEvent,
	) {
		let state = self.state.as_mut().unwrap();
		let surface_state = self.surface_state.as_mut();
		(self.event)(
			state,
			surface_state,
			Event::WindowEvent { window_id, event },
			event_loop,
		);
	}

	fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
		if let Some(state) = self.state.as_mut() {
			(self.event)(
				state,
				self.surface_state.as_mut(),
				Event::AboutToWait,
				event_loop,
			);
		}
	}
}

#[derive(Clone)]
struct Test;
impl Pipeline for Test {
	type VsOut = Colour;
	type Vertex = Point;
	type Fragment = Colour;

	fn vertex(&self, index: usize, vertex: Point) -> Self::VsOut {
		Colour::BLACK
	}

	fn fragment(&self, pos: AbsoluteScreenCoordinate, data: Self::VsOut) -> Self::Fragment {
		let (r, g, b) = hsv_to_rgb(
			360.0 - ((pos.z - 1.7) / 0.6 * 360.0).clamp(0.0, 360.0) as f64,
			1.0,
			1.0,
		);
		Colour::new(r, g, b, 255)
	}
	fn backface_culling() -> rendy3d::graphics::pipeline::back_face_culling::BackFaceCulling {
		rendy3d::graphics::pipeline::back_face_culling::BackFaceCulling::CullAnticlockwise
	}
}
pub fn entry(event_loop: EventLoop<()>) {
	let app = WinitAppBuilder::with_init(
		|elwt| {
			let window = make_window(elwt, |w| w);
			let context = softbuffer::Context::new(window.clone()).unwrap();

			#[cfg(target_arch = "wasm32")]
			{
				use winit::dpi::PhysicalSize;

				window.set_min_inner_size(Some(PhysicalSize::new(1280, 720)));
				window.set_max_inner_size(Some(PhysicalSize::new(1280, 720)));
				let w = web_sys::window().expect("there should be a window");
				let performance = w.performance().unwrap();

				return (window, context, WasmTime { performance });
			}
			#[cfg(not(target_arch = "wasm32"))]
			return (window, context, StdTime {});
		},
		|_elwt, (window, context, time)| softbuffer::Surface::new(context, window.clone()).unwrap(),
	)
	.with_event_handler(|(window, _context, time), surface, event, elwt| {
		elwt.set_control_flow(ControlFlow::Wait);

		match event {
			Event::WindowEvent {
				window_id,
				event: WindowEvent::Resized(size),
			} if window_id == window.id() => {
				debug!("{event:?}");
				let Some(surface) = surface else {
					eprintln!("Resized fired before Resumed or after Suspended");
					return;
				};

				if let (Some(width), Some(height)) =
					(NonZeroU32::new(size.width), NonZeroU32::new(size.height))
				{
					surface.resize(width, height).unwrap();
				}
			}
			Event::WindowEvent {
				window_id,
				event: WindowEvent::RedrawRequested,
			} if window_id == window.id() => {
				let Some(surface) = surface else {
					eprintln!("RedrawRequested fired before Resumed or after Suspended");
					return;
				};
				let size = window.inner_size();
				if let (Some(width), Some(height)) =
					(NonZeroU32::new(size.width), NonZeroU32::new(size.height))
				{
					// println!("Frame, width: {width}, height: {height}");
					let mut buffer = surface.buffer_mut().unwrap();
					let pixels = &mut *buffer as *mut [u32];
					let casted = unsafe { &mut *(pixels as *mut [Colour]) };
					let mut z_buffer =
						vec![f32::NEG_INFINITY; { width.get() * height.get() } as usize];
					let mut screen = Screen::new(
						casted,
						&mut z_buffer,
						width.get() as usize,
						height.get() as usize,
					);
					screen.clear(Colour::BLACK);
					let mut viewport = Viewport::new(BoundingArea2D::new(
						0,
						width.get() as usize,
						0,
						height.get() as usize,
					))
					.unwrap();
					let mut target = viewport.target(&mut screen);
					let a = Point::new(1.0, 1.0, 1.0);
					let b = Point::new(-1.0, 1.0, 1.0);
					let c = Point::new(1.0, -1.0, 1.0);
					let d = Point::new(1.0, -1.0, -1.0);
					let e = Point::new(-1.0, -1.0, 1.0);
					let f = Point::new(-1.0, -1.0, -1.0);
					let g = Point::new(-1.0, 1.0, -1.0);
					let h = Point::new(1.0, 1.0, -1.0);
					render_mesh(
						&mut target,
						&[
							Triangle::new(a, b, c),
							Triangle::new(c, b, e),
							Triangle::new(a, h, g),
							Triangle::new(b, a, g),
							Triangle::new(f, b, g),
							Triangle::new(e, b, f),
							Triangle::new(a, c, d),
							Triangle::new(h, a, d),
							Triangle::new(c, e, d),
							Triangle::new(e, f, d),
							Triangle::new(g, h, d),
							Triangle::new(f, g, d),
						],
						{
							// let x: std::time::Duration = SystemTime::now()
							// 	.duration_since(SystemTime::UNIX_EPOCH)
							// 	.unwrap();
							let x = time.secs();

							Matrix4::scale_x(height.get() as f64 / width.get() as f64)
								* Matrix4::translation(Vector3::new(0.0, 0.0, 2.0))
								* Matrix4::rotation_z(x) * Matrix4::rotation_y(x)
								* Matrix4::rotation_x(x) * Matrix4::scale(0.4)
						},
						perspective_matrix(1.0, 1.0, -20.0, 1.0),
						&mut Test {},
					);
					buffer.present().unwrap();
					window.request_redraw();
				};
			}
			Event::WindowEvent {
				event:
					WindowEvent::CloseRequested
					| WindowEvent::KeyboardInput {
						event:
							KeyEvent {
								logical_key: Key::Named(NamedKey::Escape),
								..
							},
						..
					},
				window_id,
			} if window_id == window.id() => {
				elwt.exit();
			}
			_ => {}
		}
	});

	run_app(event_loop, app);
}

pub fn start() {
	use winit::event_loop::EventLoop;

	entry(EventLoop::new().unwrap())
}
#[cfg(target_arch = "wasm32")]
use web_sys::HtmlCanvasElement;
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn start_web(window: Option<HtmlCanvasElement>) {
	use log::Level;
	console_log::init_with_level(Level::Debug);

	info!("It works!");
	// Set window to window given
	WINDOW.replace(window);
	start();
}

trait Timer {
	fn secs(&self) -> f64;
}
struct StdTime {}
impl Timer for StdTime {
	fn secs(&self) -> f64 {
		SystemTime::now()
			.duration_since(SystemTime::UNIX_EPOCH)
			.unwrap()
			.as_secs_f64()
	}
}

struct WasmTime {
	performance: Performance,
}
impl Timer for WasmTime {
	fn secs(&self) -> f64 {
		self.performance.now() / 1000.0
	}
}
