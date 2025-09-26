use crate::graphics::geometry::triangle::Triangle;

pub struct Quad<T> {
	v0: T,
	v1: T,
	v2: T,
	v3: T,
}

impl<T> Quad<T> {
	pub const fn new(v0: T, v1: T, v2: T, v3: T) -> Self {
		Self { v0, v1, v2, v3 }
	}
}
impl<T> Quad<T>
where
	T: Clone,
{
	pub fn triangulate(self) -> [Triangle<T>; 2] {
		let t0 = Triangle::new(self.v0.clone(), self.v1, self.v2.clone());
		let t1 = Triangle::new(self.v0, self.v2, self.v3);
		[t0, t1]
	}
}
