use std::ops::{Mul, MulAssign, Neg};

use crate::{
	matrices::matrix3::Matrix3,
	traits::{float::Float, num::Num},
	vector::{vector3::Vector3, vector4::Vector4},
};

#[derive(Default, Clone, Debug)]
pub struct Matrix4<T> {
	// Matrix Columns
	x: Vector4<T>,
	y: Vector4<T>,
	z: Vector4<T>,
	w: Vector4<T>,
}

impl<T> Matrix4<T> {
	pub const fn new(x: Vector4<T>, y: Vector4<T>, z: Vector4<T>, w: Vector4<T>) -> Self {
		Self { x, y, z, w }
	}
}
impl<T> Mul<Matrix4<T>> for Matrix4<T>
where
	T: Num,
{
	type Output = Matrix4<T>;
	fn mul(self, rhs: Matrix4<T>) -> Self::Output {
		let x = self.clone() * rhs.x;
		let y = self.clone() * rhs.y;
		let z = self.clone() * rhs.z;
		let w = self * rhs.w;
		Self::new(x, y, z, w)
	}
}
impl<T> Mul<Vector4<T>> for Matrix4<T>
where
	T: Num,
{
	type Output = Vector4<T>;
	fn mul(self, rhs: Vector4<T>) -> Self::Output {
		let x = self.x.x * rhs.x + self.y.x * rhs.y + self.z.x * rhs.z + self.w.x * rhs.w;
		let y = self.x.y * rhs.x + self.y.y * rhs.y + self.z.y * rhs.z + self.w.y * rhs.w;
		let z = self.x.z * rhs.x + self.y.z * rhs.y + self.z.z * rhs.z + self.w.z * rhs.w;
		let w = self.x.w * rhs.x + self.y.w * rhs.y + self.z.w * rhs.z + self.w.w * rhs.w;
		Vector4::new(x, y, z, w)
	}
}
impl<T> Matrix4<T>
where
	T: Default + Float + Neg<Output = T>,
{
	pub fn unit() -> Self {
		Self::new(
			Vector4::new(T::one(), T::zero(), T::zero(), T::zero()),
			Vector4::new(T::zero(), T::one(), T::zero(), T::zero()),
			Vector4::new(T::zero(), T::zero(), T::one(), T::zero()),
			Vector4::new(T::zero(), T::zero(), T::zero(), T::one()),
		)
	}
	pub fn with_translation(mut self, vector: Vector3<T>) -> Self {
		let mut vector4: Vector4<T> = vector.into();
		vector4.w = T::one();
		self.w = vector4;
		self
	}
	pub fn translation(vector: Vector3<T>) -> Self {
		Self::unit().with_translation(vector)
	}
	// Scale
	pub fn with_scale_x(mut self, scale: T) -> Self {
		self.x.x *= scale;
		self
	}
	pub fn scale_x(scale: T) -> Self {
		Self::unit().with_scale_x(scale)
	}
	pub fn with_scale_y(mut self, scale: T) -> Self {
		self.y.y *= scale;
		self
	}
	pub fn scale_y(scale: T) -> Self {
		Self::unit().with_scale_y(scale)
	}
	pub fn with_scale_z(mut self, scale: T) -> Self {
		self.z.z *= scale;
		self
	}
	pub fn scale_z(scale: T) -> Self {
		Self::unit().with_scale_z(scale)
	}
	pub fn with_scale(self, scale: T) -> Self {
		self.with_scale_x(scale)
			.with_scale_y(scale)
			.with_scale_z(scale)
	}
	pub fn scale(scale: T) -> Self {
		Self::unit().with_scale(scale)
	}
	pub fn rotation_x(angle: T) -> Self {
		Matrix3::rotate_x(angle).into()
	}
	pub fn rotation_y(angle: T) -> Self {
		Matrix3::rotate_y(angle).into()
	}
	pub fn rotation_z(angle: T) -> Self {
		Matrix3::rotate_z(angle).into()
	}
}

impl<T> From<Matrix3<T>> for Matrix4<T>
where
	T: Float + Default,
{
	fn from(value: Matrix3<T>) -> Self {
		Self::new(
			value.x.into(),
			value.y.into(),
			value.z.into(),
			Vector4::new(T::zero(), T::zero(), T::zero(), T::one()),
		)
	}
}

impl<T> MulAssign<T> for Matrix4<T>
where
	T: MulAssign + Clone,
{
	fn mul_assign(&mut self, rhs: T) {
		self.x *= rhs.clone();
		self.y *= rhs.clone();
		self.z *= rhs.clone();
		self.w *= rhs;
	}
}
impl<T> Mul<T> for Matrix4<T>
where
	T: MulAssign + Clone,
{
	type Output = Self;
	fn mul(mut self, rhs: T) -> Self::Output {
		self *= rhs;
		self
	}
}
