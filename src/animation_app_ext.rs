use bevy::prelude::*;

use crate::{animation_collection::AnimationCollection, animation_manager::transition_animations};

pub trait AnimationAppExt {
	fn register_animation<T: AnimationCollection>(&mut self) -> &mut Self;
}

impl AnimationAppExt for App {
	fn register_animation<T: AnimationCollection>(&mut self) -> &mut Self {
		self.add_systems(Update, transition_animations::<T>);
		self
	}
}