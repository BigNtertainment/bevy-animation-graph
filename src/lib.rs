use animation_manager::AnimationManagerPlugin;
use animation_collection::AnimationCollectionPlugin;
use bevy::prelude::*;

pub mod animation_collection;
pub mod animation_manager;
pub mod animation_graph;
pub mod animation_app_ext;
pub mod animation_bundle;

pub struct AnimationGraphPlugin;

impl Plugin for AnimationGraphPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AnimationManagerPlugin)
            .add_plugin(AnimationCollectionPlugin);
    }
}

pub mod prelude {
    pub use crate::{
        AnimationGraphPlugin,
        animation_bundle::AnimationBundle,
        animation_graph::{
            AnimationGraph, AnimationTransitionCondition,
            AnimationTransitionMode,
        },
        animation_manager::AnimationManager,
    };
    
    #[cfg(feature = "spritesheet_animation")]
    pub use crate::animation_collection::spritesheet_animation::{
        AnimationBounds, SpritesheetAnimation, SpritesheetAnimationCollection,
    };
}
