use bevy::prelude::*;

/// Trait used for implementing your own animation types.
pub trait AnimationCollection : Component {
    fn transition(&mut self, animation_index: usize);
    fn is_current_animation_finished(&self) -> bool;
}
