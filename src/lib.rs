use animation_manager::AnimationManagerPlugin;
use bevy::prelude::*;

pub mod animation;
pub mod animation_manager;
pub mod animation_graph;

pub struct SpritesheetAnimationPlugin;

impl Plugin for SpritesheetAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AnimationManagerPlugin);
    }
}
