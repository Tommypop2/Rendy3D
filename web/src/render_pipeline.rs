use hsv::hsv_to_rgb;
use rendy3d::graphics::{
	colour::Colour,
	geometry::{clipping::SutherlandHodgman, point::AbsoluteScreenCoordinate},
	geometry_3d::point::Point,
	interpolate::PerspectiveCorrectInterpolate,
	pipeline::Pipeline,
};

pub struct WebDemo;
impl Pipeline for WebDemo {
	type VsOut = PerspectiveCorrectInterpolate<f64>;
	type Vertex = Point;
	type Fragment = Colour;
	type ClippingStrategy = SutherlandHodgman;

	fn vertex(&self, _index: usize, vertex: Point) -> (Point, Self::VsOut) {
		// let (r, g, b) = hsv_to_rgb(
		// 	360.0 - ((vertex.z - 1.5) / 2.0 * 360.0).clamp(0.0, 360.0) as f64,
		// 	1.0,
		// 	1.0,
		// );
		// let c = Colour::new(r, g, b, 255);
		(
			vertex,
			PerspectiveCorrectInterpolate::new(360.0 * (1.0 - ((vertex.z + 2.8) / 3.0)), vertex.z),
		)
	}

	fn fragment(&self, _pos: AbsoluteScreenCoordinate, data: Self::VsOut) -> Self::Fragment {
		// let (r, g, b) = hsv_to_rgb(
		// 	360.0 - ((pos.z - 1.7) / 0.6 * 360.0).clamp(0.0, 360.0) as f64,
		// 	1.0,
		// 	1.0,
		// );
		// Colour::new(r, g, b, 255)
		let hue = data.get();
		let (r, g, b) = hsv_to_rgb(hue.clamp(0.0, 360.0), 1.0, 1.0);
		Colour::new(r, g, b, 255)
	}
	fn backface_culling() -> rendy3d::graphics::pipeline::back_face_culling::BackFaceCulling {
		rendy3d::graphics::pipeline::back_face_culling::BackFaceCulling::None
	}
}
