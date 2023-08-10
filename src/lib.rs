use animation_collection::AnimationCollectionPlugin;
use animation_manager::AnimationManagerPlugin;
use bevy::prelude::*;

pub mod animation_app_ext;
pub mod animation_bundle;
pub mod animation_collection;
pub mod animation_graph;
pub mod animation_manager;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub enum AnimationGraph {
    Execute,
    Transition,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnimationGraphPlugin;

impl Plugin for AnimationGraphPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AnimationManagerPlugin)
            .add_plugins(AnimationCollectionPlugin);
    }
}

pub mod prelude {
    pub use crate::{
        animation_bundle::AnimationBundle,
        animation_graph::{AnimationGraph, AnimationTransitionCondition, AnimationTransitionMode},
        animation_manager::AnimationManager,
        AnimationGraphPlugin,
    };

    #[cfg(feature = "spritesheet_animation")]
    pub use crate::animation_collection::spritesheet_animation::{
        AnimationBounds, SpritesheetAnimation, SpritesheetAnimationCollection,
    };
}
