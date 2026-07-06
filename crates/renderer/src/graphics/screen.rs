use core::{f32, slice};

use rendy3d_maths::geometry::bounding_area::BoundingArea2D;

use crate::graphics::{colour::Colour, target::Target};

/// Root [`Target`] that actually writes to a framebuffer
///
/// This can be replaced by anything else that implements [`Target`]
pub struct Screen<'a, Pixel> {
	pub frame_buffer: &'a mut [Pixel],
	width: usize,
	height: usize,
	pub z_buffer: &'a mut [f32],
	pub draw_colour: Pixel,
}
impl<'a, Pixel> Target for Screen<'a, Pixel>
where
	Pixel: Clone + Default,
{
	type Item = Pixel;

	fn set(&mut self, x: usize, y: usize, value: Self::Item) {
		let w = self.width;
		self.frame_mut()[y * w + x] = value
	}

	fn get(&self, x: usize, y: usize) -> Self::Item {
		self.frame()[y * self.width + x].clone()
	}

	fn set_depth(&mut self, x: usize, y: usize, value: f32) {
		self.z_buffer[y * self.width + x] = value
	}

	fn get_depth(&self, x: usize, y: usize) -> f32 {
		self.z_buffer[y * self.width + x]
	}

	fn clear(&mut self, fill: Self::Item) {
		self.clear_depth();
		self.frame_mut().fill(fill);
	}

	fn clear_depth(&mut self) {
		self.reset_z_buffer();
	}

	fn area(&self) -> BoundingArea2D {
		BoundingArea2D {
			min_x: 0,
			max_x: self.width,
			min_y: 0,
			max_y: self.height,
		}
	}

	fn draw_colour(&self) -> Self::Item {
		self.draw_colour.clone()
	}
	fn set_draw_colour(&mut self, value: Self::Item) {
		self.draw_colour = value;
	}
}
impl<'a, Pixel> Screen<'a, Pixel> where Pixel: Clone + Default {
	pub fn new(
		frame_buffer: &'a mut [Pixel],
		z_buffer: &'a mut [f32],
		width: usize,
		height: usize,
	) -> Self {
		Self {
			frame_buffer,
			z_buffer,
			width,
			height,
			draw_colour: Pixel::default(),
		}
	}
	#[inline]
	pub fn frame(&self) -> &[Pixel] {
		self.frame_buffer
	}
	#[inline]
	pub fn frame_mut(&mut self) -> &mut [Pixel] {
		self.frame_buffer
	}
	pub fn reset_z_buffer(&mut self) {
		self.z_buffer.fill(f32::INFINITY);
	}
}

#[inline]
pub const fn frame_pixels(frame: &mut [u8]) -> &mut [Colour] {
	// SAFETY: Format for each pixel matches the layout of the `Colour` struct (and is 4 bytes)
	// mem::transmute doesn't work here as it doesn't adjust the length of the slice, even though it is transmuted into a 2D array (so the length reduces)

	// (unsafe {
	// 	let ptr = frame as *mut [u8];
	// 	let casted = ptr as *mut [[Colour; WIDTH as usize]; HEIGHT as usize];
	// 	&mut *casted
	// }) as _

	// unsafe { &mut *(frame as *mut [u8] as *mut [Colour]) }
	unsafe { slice::from_raw_parts_mut(frame as *mut [u8] as *mut Colour, frame.len() / 4) }
}
