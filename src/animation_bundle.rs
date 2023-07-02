use bevy::prelude::*;

use crate::{animation_collection::AnimationCollection, animation_manager::AnimationManager};

#[derive(Bundle, Debug, Default)]
pub struct AnimationBundle<T: AnimationCollection> {
	pub animation_collection: T,
	pub animation_manager: AnimationManager,
}
