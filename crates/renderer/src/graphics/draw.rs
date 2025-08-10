use crate::graphics::{shaders::shaders::Shaders, target::Target};

pub trait Draw {
	fn draw<T: Target, S: Shaders + Clone>(&self, target: &mut T, shaders: S);
}
