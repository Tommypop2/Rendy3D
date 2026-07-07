use hsv::hsv_to_rgb;
use rendy3d::{
	graphics::{
		colour::Colour,
		geometry::{clipping::SutherlandHodgman, point::AbsoluteScreenCoordinate},
		geometry_3d::point::Point,
		interpolate::PerspectiveCorrectInterpolate,
		pipeline::Pipeline,
	},
	maths::{matrices::matrix4::Matrix4, vector::vector3::Vector3},
};

pub struct WebDemo;
impl Pipeline for WebDemo {
	type VsOut = PerspectiveCorrectInterpolate<f64>;
	type VsIn = (f64, f64);
	type Vertex = Point;
	type Fragment = Colour;
	type ClippingStrategy = SutherlandHodgman;

	fn vertex(&self, _index: usize, vertex: Point, state: Self::VsIn) -> (Point, Self::VsOut) {
		// Apply transformation & projection matrices
		let (x, aspect_ratio) = state;
		let mat = Matrix4::scale_x(aspect_ratio)
			* Matrix4::translation(Vector3::new(0.0, 0.0, -2.0))
			* Matrix4::rotation_z(x)
			* Matrix4::rotation_y(x)
			* Matrix4::rotation_x(x)
			* Matrix4::scale(0.4);
		let vertex = vertex.apply(mat);
		(
			vertex.apply(Matrix4::new_perspective(1.0, 1.0, 20.0, 0.1)),
			PerspectiveCorrectInterpolate::new(360.0 * (1.0 - ((vertex.z + 2.8) / 3.0)), vertex.z),
		)
	}

	fn fragment(&self, _pos: AbsoluteScreenCoordinate, data: Self::VsOut) -> Self::Fragment {
		let hue = data.get();
		let (r, g, b) = hsv_to_rgb(hue.clamp(0.0, 360.0), 1.0, 1.0);
		Colour::new(r, g, b, 255)
	}
	fn backface_culling() -> rendy3d::graphics::pipeline::back_face_culling::BackFaceCulling {
		rendy3d::graphics::pipeline::back_face_culling::BackFaceCulling::None
	}
}
