use crate::graphics::target::Target;

pub trait Draw {
	fn draw<T: Target>(&self, target: &mut T);
}
