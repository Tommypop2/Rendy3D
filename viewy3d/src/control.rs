use std::collections::HashMap;

use rendy3d::{
	graphics::camera::Camera,
	maths::{matrices::matrix4::Matrix4, vector::vector3::Vector3},
};
use winit::{
	event::{DeviceEvent, ElementState, MouseButton, RawKeyEvent, WindowEvent},
	keyboard::{KeyCode, PhysicalKey},
};

pub struct FirstPersonControl {
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
	pub fn handle_window_event(&mut self, event: &WindowEvent) {
		if let WindowEvent::MouseInput {
			device_id: _,
			state,
			button: MouseButton::Left,
		} = event
		{
			self.click_pressed = match state {
				ElementState::Pressed => true,
				ElementState::Released => false,
			}
		}
	}
	pub fn handle_device_event(&mut self, event: &DeviceEvent, camera: &mut Camera) {
		match event {
			DeviceEvent::MouseMotion { delta } => {
				if !self.click_pressed {
					return;
				}
				let dx = delta.0 * self.speed;
				let dy = delta.1 * self.speed;
				// Ideally, this wouldn't be done here. Instead, it should be done in `step`
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
	pub fn step(&mut self, camera: &mut Camera) {
		for (code, pressed) in self.keys_pressed.iter() {
			if !*pressed {
				continue;
			}
			let transform = match code {
				KeyCode::KeyW => Matrix4::translation(Vector3::new(0.0, 0.0, -self.speed)),
				KeyCode::KeyS => Matrix4::translation(Vector3::new(0.0, 0.0, self.speed)),
				KeyCode::KeyA => Matrix4::translation(Vector3::new(-self.speed, 0.0, 0.0)),
				KeyCode::KeyD => Matrix4::translation(Vector3::new(self.speed, 0.0, 0.0)),
				KeyCode::Space => Matrix4::translation(Vector3::new(0.0, self.speed, 0.0)),
				KeyCode::ShiftLeft | KeyCode::ShiftRight => {
					Matrix4::translation(Vector3::new(0.0, -self.speed, 0.0))
				}
				_ => {
					continue;
				}
			};
			camera.transformation = camera.transformation.clone() * transform;
		}
	}
}
