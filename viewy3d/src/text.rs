use rendy3d::graphics::{colour::Colour, shapes_2d::point::AbsoluteScreenCoordinate};

pub fn draw_char(
	font: &fontdue::Font,
	offset: AbsoluteScreenCoordinate,
	frame_buffer: &mut [Colour],
	ch: char,
	size: f32,
	width: usize,
) -> fontdue::Metrics {
	let (metrics, bitmap) = font.rasterize(ch, size);
	if metrics.width == 0 {
		return metrics;
	}
	let rows = bitmap.chunks(metrics.width);
	for (row_index, row) in rows.into_iter().enumerate() {
		let height_offset = (row_index + offset.y) * width;
		for (j, pixel) in frame_buffer
			[(height_offset + offset.x)..(height_offset + offset.x + row.len())]
			.iter_mut()
			.enumerate()
		{
			let v = row[j];
			*pixel = Colour::new(v, v, v, 255)
		}
	}
	metrics
}
pub fn draw_text(
	font: &fontdue::Font,
	mut offset: AbsoluteScreenCoordinate,
	frame_buffer: &mut [Colour],
	text: &str,
	size: f32,
	width: usize,
) {
	for ch in text.chars() {
		offset.x += draw_char(font, offset, frame_buffer, ch, size, width).advance_width as usize
	}
}
