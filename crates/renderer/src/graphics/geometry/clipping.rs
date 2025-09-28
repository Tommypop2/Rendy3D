//! Implementation of a triangle clipping algorithm

use rendy3d_maths::{geometry::{quad::Quad, triangle::Triangle}, vector::vector4::Vector4};

use crate::graphics::{
	interpolate::Interpolate,
};
#[derive(Clone, Copy)]
pub enum ClippingPlane {
	NearZ,
	FarZ,
	PosX,
	NegX,
	PosY,
	NegY,
}
impl ClippingPlane {
	pub fn clipping_planes() -> [Self; 6] {
		[
			Self::NearZ,
			Self::FarZ,
			Self::PosX,
			Self::NegX,
			Self::PosY,
			Self::NegY,
		]
	}
	pub fn equation(self) -> Vector4<f64> {
		match self {
			Self::NearZ => Vector4::new(0.0, 0.0, 1.0, 1.0),
			Self::FarZ => Vector4::new(0.0, 0.0, -1.0, 1.0),
			Self::PosX => Vector4::new(-1.0, 0.0, 0.0, 1.0),
			Self::NegX => Vector4::new(1.0, 0.0, 0.0, 1.0),
			Self::PosY => Vector4::new(0.0, -1.0, 0.0, 1.0),
			Self::NegY => Vector4::new(0.0, 1.0, 0.0, 1.0),
		}
	}
}
pub trait TriangleClipper<T> {
	type ClipResult: IntoIterator<Item = Option<Triangle<Vertex<T>>>>;
	/// Clips triangle by each equation in turn
	fn clip(triangle: Triangle<Vertex<T>>) -> impl Iterator<Item = Triangle<Vertex<T>>>;
	/// Clips triangle by a single equation
	fn clip_equation(triangle: Triangle<Vertex<T>>, equation: Vector4<f64>) -> Self::ClipResult;
}

pub struct SutherlandHodgman;
type Vertex<T> = (Vector4<f64>, T);
impl SutherlandHodgman {
	/// Compute intersection point with plane
	fn intersection<T: Interpolate>(
		v0: &Vertex<T>,
		v1: &Vertex<T>,
		value0: f64,
		value1: f64,
	) -> Vertex<T> {
		let t = value0 / (value0 - value1);
		let u = 1.0 - t;
		// Interpolate vertices
		let p = v0.0 + (v1.0 - v0.0) * t;

		// Interpolate attributes
		let a = T::interpolate2(&v0.1, &v1.1, t as f32, u as f32);

		(p, a)
	}
}
impl<T> TriangleClipper<T> for SutherlandHodgman
where
	T: Interpolate + Clone,
{
	type ClipResult = [Option<Triangle<Vertex<T>>>; 2];
	fn clip(triangle: Triangle<Vertex<T>>) -> impl Iterator<Item = Triangle<Vertex<T>>> {
		Self::clip_equation(triangle, ClippingPlane::NearZ.equation())
			.into_iter()
			.flatten()
			.flat_map(|triangle| Self::clip_equation(triangle, ClippingPlane::FarZ.equation()))
			.flatten()
			.flat_map(|t| Self::clip_equation(t, ClippingPlane::NegX.equation()))
			.flatten()
			.flat_map(|t| Self::clip_equation(t, ClippingPlane::PosX.equation()))
			.flatten()
			.flat_map(|t| Self::clip_equation(t, ClippingPlane::NegY.equation()))
			.flatten()
			.flat_map(|t| Self::clip_equation(t, ClippingPlane::PosY.equation()))
			.flatten()
	}
	fn clip_equation(triangle: Triangle<Vertex<T>>, equation: Vector4<f64>) -> Self::ClipResult {
		// let near_plane = Vector4::new(0.0, 0.0, 1.0, 1.0);
		let v0 = triangle.vertex1;
		let v1 = triangle.vertex2;
		let v2 = triangle.vertex3;
		// Test each vertex
		let values = [
			v0.0.dot_with(equation),
			v1.0.dot_with(equation),
			v2.0.dot_with(equation),
		];
		// println!("{values:?}");
		let mask: u8 =
			(values[0] < 0.0) as u8 | ((values[1] < 0.0) as u8 * 2) | ((values[2] < 0.0) as u8 * 4);
		let out_triangles: [Option<Triangle<Vertex<T>>>; 2] = match mask {
			0b000 => {
				// All triangles are in range, so no clipping :(
				[Some(Triangle::new(v0, v1, v2)), None]
			}
			0b001 => {
				// V0 out of range
				let nv0 = Self::intersection(&v0, &v1, values[0], values[1]);
				let nv1 = Self::intersection(&v0, &v2, values[0], values[2]);

				// Triangle::new(nv0.clone(), v1, v2.clone());
				// Triangle::new(nv0, v2, nv1);
				let [x, y] = Quad::new(nv0, v1, v2, nv1).triangulate();
				[Some(x), Some(y)]
			}
			0b010 => {
				// V1 out of range
				let nv0 = Self::intersection(&v1, &v0, values[1], values[0]);
				let nv1 = Self::intersection(&v1, &v2, values[1], values[2]);
				// out_triangles.extend_from_slice(&q.triangulate());
				let [x, y] = Quad::new(nv0, nv1, v2, v0).triangulate();
				[Some(x), Some(y)]
			}
			0b011 => {
				// V0 and V1 out of range
				let nv0 = Self::intersection(&v2, &v0, values[2], values[0]);
				let nv1 = Self::intersection(&v2, &v1, values[2], values[1]);
				let t = Triangle::new(nv0, nv1, v2);
				[Some(t), None]
			}
			0b100 => {
				// V2 out of range
				let nv0 = Self::intersection(&v2, &v0, values[2], values[0]);
				let nv1 = Self::intersection(&v2, &v1, values[2], values[1]);
				let [x, y] = Quad::new(v0, v1, nv1, nv0).triangulate();
				[Some(x), Some(y)]
			}
			0b101 => {
				// V0 and V2 out of range
				let nv0 = Self::intersection(&v1, &v0, values[1], values[0]);
				let nv1 = Self::intersection(&v1, &v2, values[1], values[2]);
				let t = Triangle::new(v1, nv1, nv0);
				[Some(t), None]
			}
			0b110 => {
				// V1 and V2 out of range
				let nv0 = Self::intersection(&v0, &v1, values[0], values[1]);
				let nv1 = Self::intersection(&v0, &v2, values[0], values[2]);
				let t = Triangle::new(v0, nv0, nv1);
				[Some(t), None]
			}
			0b111 => {
				// All vertices out of range, no need to generate any triangles
				[None, None]
			}
			_ => unreachable!(),
		};
		out_triangles
	}
}

// pub struct NoClip;
// impl<T> TriangleClipper<T> for NoClip {
// 	type ClipResult = ();

// 	fn clip(triangle: Triangle<Vertex<T>>) -> impl Iterator<Item = Triangle<Vertex<T>>> {
// 		todo!()
// 	}

// 	fn clip_equation(triangle: Triangle<Vertex<T>>, equation: Vector4<f64>) -> Self::ClipResult {
// 		todo!()
// 	}
// }
