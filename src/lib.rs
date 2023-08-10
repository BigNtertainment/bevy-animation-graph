use animation_manager::AnimationManagerPlugin;
use bevy::prelude::*;
use spritesheet_animation::SpritesheetAnimationPlugin;

pub mod animation_collection;
pub mod animation_manager;
pub mod animation_graph;
pub mod animation_app_ext;
pub mod animation_bundle;

pub mod spritesheet_animation;

pub struct AnimationGraphPlugin;

impl Plugin for AnimationGraphPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AnimationManagerPlugin)
            .add_plugin(SpritesheetAnimationPlugin);
    }
}

pub mod prelude {
    pub use crate::{
        animation_app_ext::AnimationAppExt,
        animation_bundle::AnimationBundle,
        animation_graph::{
            AnimationGraph, AnimationTransitionCondition,
            AnimationTransitionMode,
        },
        animation_manager::AnimationManager,
        spritesheet_animation::{
            AnimationBounds, SpritesheetAnimation, SpritesheetAnimationCollection,
            SpritesheetAnimationPlugin,
        },
    };
}
