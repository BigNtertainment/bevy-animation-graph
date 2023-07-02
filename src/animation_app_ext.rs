use bevy::prelude::*;

use crate::{animation_collection::AnimationCollection, animation_manager::transition_animations};

pub trait AnimationAppExt {
	fn register_animation<T: AnimationCollection>(&mut self) -> &mut Self;
}

impl AnimationAppExt for App {
	fn register_animation<T: AnimationCollection>(&mut self) -> &mut Self {
		self.add_system(transition_animations::<T>);
		self
	}
}