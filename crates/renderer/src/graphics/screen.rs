use std::f32;

use fixed_capacity_vec::FixedCapacityVec;
use pixels::Pixels;

use crate::{
	HEIGHT, WIDTH,
	graphics::{colour::Colour, shapes_2d::point::AbsoluteScreenCoordinate},
};

pub struct Screen<'a> {
	pub pixels: Pixels<'a>,
	pub z_buffer: Box<[Box<[f32; WIDTH as usize]>; HEIGHT as usize]>,
	pub draw_colour: Colour,
}

impl<'a> Screen<'a> {
	pub fn new(pixels: Pixels<'a>) -> Self {
		Self {
			pixels,
			z_buffer: {
				let mut data: FixedCapacityVec<Box<[f32; WIDTH as usize]>, { (HEIGHT) as usize }> =
					FixedCapacityVec::new();
				while data
					.try_push({
						let mut data: FixedCapacityVec<f32, { WIDTH as usize }> =
							FixedCapacityVec::new();
						while data.try_push(f32::NEG_INFINITY).is_ok() {}
						let boxed: Box<[f32; WIDTH as usize]> = data.try_into().unwrap();
						boxed
					})
					.is_ok()
				{}
				let boxed: Box<[Box<[f32; WIDTH as usize]>; HEIGHT as usize]> =
					data.try_into().unwrap();
				boxed
			},
			draw_colour: Colour::new(0x48, 0xb2, 0xe8, 255),
		}
	}
	pub fn frame(&mut self) -> &mut [[Colour; WIDTH as usize]] {
		frame_pixels(self.pixels.frame_mut())
	}
	pub fn set_draw_colour(&mut self, colour: Colour) {
		self.draw_colour = colour;
	}
	pub fn draw_point(&mut self, p: AbsoluteScreenCoordinate) {
		self.frame()[p.y][p.x] = self.draw_colour.clone();
	}
	pub fn get_z_in_z_buffer(&self, p: AbsoluteScreenCoordinate) -> f32 {
		self.z_buffer[p.y][p.x]
	}
	pub fn reset_z_buffer(&mut self) {
		for row in self.z_buffer.iter_mut() {
			row.fill(f32::NEG_INFINITY);
		}
	}
	pub fn clear(&mut self, colour: Colour) {
		self.reset_z_buffer();
		self.frame().as_flattened_mut().fill(colour);
	}
}

const fn frame_pixels(frame: &mut [u8]) -> &mut [[Colour; WIDTH as usize]] {
	// SAFETY: Format for each pixel matches the layout of the `Colour` struct (and is 4 bytes)
	// mem::transmute doesn't work here as it doesn't adjust the length of the slice, even though it is transmuted into a 2D array (so the length reduces)

	(unsafe {
		let ptr = frame as *mut [u8];
		let casted = ptr as *mut [[Colour; WIDTH as usize]; HEIGHT as usize];
		&mut *casted
	}) as _
}
