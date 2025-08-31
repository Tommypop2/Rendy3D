use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

// Base number trait
pub trait Num:
	Sized
	+ PartialEq
	+ Mul<Output = Self>
	+ MulAssign
	+ Div<Output = Self>
	+ DivAssign
	+ Add<Output = Self>
	+ AddAssign
	+ Sub<Output = Self>
	+ SubAssign
	+ Copy
	+ Clone
{
	fn one() -> Self;
	fn zero() -> Self;
}
macro_rules! implement_num {
	($x: ident) => {
		impl Num for $x {
			fn one() -> Self {
				1 as Self
			}
			fn zero() -> Self {
				0 as Self
			}
		}
	};
}

implement_num!(f32);
implement_num!(f64);

implement_num!(i8);
implement_num!(i16);
implement_num!(i32);
implement_num!(i64);
implement_num!(i128);

implement_num!(u8);
implement_num!(u16);
implement_num!(u32);
implement_num!(u64);
implement_num!(u128);

implement_num!(usize);