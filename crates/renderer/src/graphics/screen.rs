use core::f32;

use crate::{
	HEIGHT, WIDTH,
	graphics::{colour::Colour, shapes_2d::point::AbsoluteScreenCoordinate},
};

pub struct Screen<'a> {
	pub frame_buffer: &'a mut [[Colour; WIDTH as usize]],
	pub z_buffer: &'a mut [f32],
	pub draw_colour: Colour,
}

impl<'a> Screen<'a> {
	pub fn new(frame_buffer: &'a mut [[Colour; WIDTH as usize]], z_buffer: &'a mut [f32]) -> Self {
		Self {
			frame_buffer,
			z_buffer,
			draw_colour: Colour::new(0x48, 0xb2, 0xe8, 255),
		}
	}
	pub fn frame(&mut self) -> &mut [[Colour; WIDTH as usize]] {
		// frame_pixels(self.frame_buffer.frame_mut())
		self.frame_buffer
	}
	pub fn set_draw_colour(&mut self, colour: Colour) {
		self.draw_colour = colour;
	}
	pub fn draw_point(&mut self, p: AbsoluteScreenCoordinate) {
		self.frame()[p.y][p.x] = self.draw_colour.clone();
	}
	pub fn set_z_in_z_buffer(&mut self, p: AbsoluteScreenCoordinate) {
		self.z_buffer[p.y * HEIGHT as usize + p.x] = p.z
	}
	pub fn get_z_in_z_buffer(&self, p: AbsoluteScreenCoordinate) -> f32 {
		self.z_buffer[p.y * HEIGHT as usize + p.x]
	}
	pub fn reset_z_buffer(&mut self) {
		let b = unsafe {
			let ptr = self.z_buffer as *mut [f32] as *mut [f32; { WIDTH * HEIGHT } as usize];
			&mut *ptr
		};
		b.fill(f32::NEG_INFINITY);
	}
	pub fn clear(&mut self, colour: Colour) {
		self.reset_z_buffer();
		self.frame().as_flattened_mut().fill(colour);
	}
}

#[inline]
pub const fn frame_pixels(frame: &mut [u8]) -> &mut [[Colour; WIDTH as usize]] {
	// SAFETY: Format for each pixel matches the layout of the `Colour` struct (and is 4 bytes)
	// mem::transmute doesn't work here as it doesn't adjust the length of the slice, even though it is transmuted into a 2D array (so the length reduces)

	(unsafe {
		let ptr = frame as *mut [u8];
		let casted = ptr as *mut [[Colour; WIDTH as usize]; HEIGHT as usize];
		&mut *casted
	}) as _
}
