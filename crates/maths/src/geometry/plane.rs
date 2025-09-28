use crate::vector::vector3::Vector3;

/// Represents a plane in the form ax + by + cz = d
pub struct Plane {
	normal: Vector3<f64>,
	distance: f64,
}

impl Plane {
	pub fn new(normal: Vector3<f64>, distance: f64) -> Self {
		let m = 1.0 / normal.magnitude();
		Self {
			// Normalize on creation to avoid expensive normalization when calculating the distance
			normal: normal * m,
			distance: distance * m,
		}
	}
	pub fn signed_distance(&self, p: Vector3<f64>) -> f64 {
		let (x, y, z) = p.as_tuple();
		let (a, b, c) = self.normal.as_tuple();
		a * x + b * y + c * z - self.distance
	}
	pub fn distance(&self, p: Vector3<f64>) -> f64 {
		self.signed_distance(p).abs()
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn distance() {
		let p = Vector3::new(3.0, 2.0, -1.0);
		let plane = Plane::new(Vector3::new(2.0, -3.0, 1.0), 5.0);
		let distance = plane.distance(p);
		assert!(distance - 6.0 / (f64::sqrt(14.0)) <= 1E-10)
	}
}
