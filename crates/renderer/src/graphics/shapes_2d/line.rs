use crate::graphics::{
	draw::Draw, interpolate::Interpolate, shaders::shaders::Shaders,
	shapes_2d::point::AbsoluteScreenCoordinate, target::Target,
};

pub struct Line {
	start: AbsoluteScreenCoordinate,
	end: AbsoluteScreenCoordinate,
}
impl Line {
	pub fn new(start: AbsoluteScreenCoordinate, end: AbsoluteScreenCoordinate) -> Self {
		Self { start, end }
	}
	fn draw_line_low<T: Target, U: Interpolate, S: Shaders<VsOut = U, Pixel = T::Item> + Clone>(
		target: &mut T,
		shaders: &mut S,
		start: AbsoluteScreenCoordinate,
		end: AbsoluteScreenCoordinate,
	) {
		let min_z = start.z.min(end.z);
		let dx = (end.x - start.x) as i32;
		let (dy, yi) = {
			let dy = end.y as i32 - start.y as i32;
			if dy < 0 { (-dy, -1) } else { (dy, 1) }
		};
		let mut d = 2 * dy - dx;
		let mut y = start.y as i32;
		for x in start.x..=end.x {
			AbsoluteScreenCoordinate::new(x, y as usize, min_z).draw(target, shaders);
			if d > 0 {
				y += yi;
				d += 2 * (dy - dx)
			} else {
				d += 2 * dy
			}
		}
	}
	fn draw_line_high<T: Target, U: Interpolate, S: Shaders<VsOut = U, Pixel = T::Item> + Clone>(
		target: &mut T,
		shaders: &mut S,
		start: AbsoluteScreenCoordinate,
		end: AbsoluteScreenCoordinate,
	) {
		let min_z = start.z.min(end.z);
		let (dx, xi) = {
			let dx = end.x as i32 - start.x as i32;
			if dx < 0 { (-dx, -1) } else { (dx, 1) }
		};
		let dy = (end.y - start.y) as i32;
		let mut d = 2 * dx - dy;
		let mut x = start.x as i32;
		for y in start.y..=end.y {
			AbsoluteScreenCoordinate::new(x as usize, y, min_z).draw(target, shaders);
			if d > 0 {
				x += xi;
				d += 2 * (dx - dy)
			} else {
				d += 2 * dx
			}
		}
	}
	pub fn draw_line<T: Target, U: Interpolate, S: Shaders<VsOut = U, Pixel = T::Item> + Clone>(
		&self,
		target: &mut T,
		shaders: &mut S,
		start: AbsoluteScreenCoordinate,
		end: AbsoluteScreenCoordinate,
	) {
		if usize::abs_diff(start.y, end.y) < usize::abs_diff(start.x, end.x) {
			if end.x > start.x {
				Self::draw_line_low(target, shaders, start, end);
			} else {
				Self::draw_line_low(target, shaders, end, start);
			}
		} else if end.y > start.y {
			Self::draw_line_high(target, shaders, start, end);
		} else {
			Self::draw_line_high(target, shaders, end, start);
		}
	}
}
impl<VsOut: Interpolate> Draw<VsOut> for Line {
	fn draw<T: Target, S: Shaders<VsOut = VsOut, Pixel = T::Item> + Clone>(
		&self,
		target: &mut T,
		shaders: &mut S,
	) {
		if target.point_below_z_buffer(self.start) && target.point_below_z_buffer(self.end) {
			return;
		}
		self.draw_line(target, shaders, self.start, self.end);
	}
}
