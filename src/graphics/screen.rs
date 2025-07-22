use std::ops::Deref;

use derive_more::{Add, Deref, DerefMut};
use fixed_capacity_vec::FixedCapacityVec;
use pixels::Pixels;

use crate::{
	HEIGHT, WIDTH, frame_pixels,
	graphics::{colour::Colour, shapes_2d::point::PixelCoordinate},
};

pub struct Screen<'a> {
	pub pixels: Pixels<'a>,
	pub z_buffer: Box<[Box<[u16; WIDTH as usize]>; HEIGHT as usize]>,
	pub draw_colour: Colour,
}

impl<'a> Screen<'a> {
	pub fn new(pixels: Pixels<'a>) -> Self {
		Self {
			pixels,
			z_buffer: {
				let mut data: FixedCapacityVec<Box<[u16; WIDTH as usize]>, { (HEIGHT) as usize }> =
					FixedCapacityVec::new();
				while let Ok(_) = data.try_push({
					let mut data: FixedCapacityVec<u16, { WIDTH as usize }> =
						FixedCapacityVec::new();
					while let Ok(_) = data.try_push(0) {}
					let boxed: Box<[u16; WIDTH as usize]> = data.try_into().unwrap();
					boxed
				}) {}
				let boxed: Box<[Box<[u16; WIDTH as usize]>; HEIGHT as usize]> =
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
	pub fn clear(&mut self, colour: Colour) {
		self.frame().as_flattened_mut().fill(colour);
	}
}
