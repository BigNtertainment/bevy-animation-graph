use bevy::prelude::*;

#[cfg(feature = "spritesheet_animation")]
use spritesheet_animation::SpritesheetAnimationPlugin;

#[cfg(feature = "spritesheet_animation")]
pub mod spritesheet_animation;

/// Trait used for implementing your own animation types.
pub trait AnimationCollection : Component {
    fn transition(&mut self, animation_index: usize);
    fn is_current_animation_finished(&self) -> bool;
}

pub(crate) struct AnimationCollectionPlugin;

impl Plugin for AnimationCollectionPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "spritesheet_animation")]
        app.add_plugins(SpritesheetAnimationPlugin);
    }
}
