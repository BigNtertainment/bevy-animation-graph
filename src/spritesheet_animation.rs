use std::time::Duration;

use bevy::prelude::*;

use crate::{animation_app_ext::AnimationAppExt, animation_collection::AnimationCollection, animation_bundle::AnimationBundle};

pub struct SpritesheetAnimationPlugin;

impl Plugin for SpritesheetAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.register_animation::<SpritesheetAnimationCollection>()
            .add_system(execute_spritesheet_animation);
    }
}

#[derive(Debug, Default, Clone, Copy, Reflect, PartialEq, Eq)]
pub struct AnimationBounds {
    pub first_frame_index: usize,
    pub last_frame_index: usize,
}

impl AnimationBounds {
    pub fn new(first_frame_index: usize, last_frame_index: usize) -> Self {
        Self {
            first_frame_index,
            last_frame_index,
        }
    }
}

#[derive(Debug, Default, Clone, Reflect)]
pub struct SpritesheetAnimation {
    // TODO: Add support for multiple spritesheets
    bounds: AnimationBounds,
    frame_duration: Duration,
}

impl SpritesheetAnimation {
    pub fn new(bounds: AnimationBounds, frame_duration: Duration) -> Self {
        Self {
            bounds,
            frame_duration,
        }
    }
}

#[derive(Component, Debug, Default, Clone)]
pub struct SpritesheetAnimationCollection {
    pub animations: Vec<SpritesheetAnimation>,
    pub current_animation_index: usize,
    pub current_frame_index: usize,
    pub frame_timer: Timer,
}

impl SpritesheetAnimationCollection {
    pub fn new(animations: Vec<SpritesheetAnimation>) -> Self {
        assert!(
            !animations.is_empty(),
            "spritesheet animation collection must contain at least one animation"
        );

        let frame_timer_duration = animations[0].frame_duration;

        Self {
            animations,
            current_animation_index: 0,
            current_frame_index: 0,
            frame_timer: Timer::new(
                frame_timer_duration,
                TimerMode::Repeating,
            ),
        }
    }

    pub fn current_spritesheet_animation(&self) -> &SpritesheetAnimation {
        &self.animations[self.current_animation_index]
    }
}

pub type SpritesheetAnimationBundle = AnimationBundle<SpritesheetAnimation>;

impl AnimationCollection for SpritesheetAnimationCollection {
    fn transition(&mut self, animation_index: usize) {
        assert!(
            animation_index < self.animations.len(),
            "spritesheet animation collection doesn't contain animation with index {}",
            animation_index
        );

        self.current_animation_index = animation_index;
        self.current_frame_index = self
            .current_spritesheet_animation()
            .bounds
            .first_frame_index;
        self.frame_timer = Timer::new(
            self.current_spritesheet_animation().frame_duration,
            TimerMode::Repeating,
        );
    }

    fn is_current_animation_finished(&self) -> bool {
        self.current_frame_index == self.current_spritesheet_animation().bounds.last_frame_index + 1
    }
}

fn execute_spritesheet_animation(
    mut query: Query<(&mut SpritesheetAnimationCollection, &mut TextureAtlasSprite)>,
    time: Res<Time>,
) {
    for (mut animation_collection, mut texture_atlas_sprite) in query.iter_mut() {
        if animation_collection.is_current_animation_finished() {
            continue;
        }

        animation_collection.frame_timer.tick(time.delta());

        if animation_collection.frame_timer.finished() {
            animation_collection.current_frame_index += 1;
        }

        if animation_collection.current_frame_index
            <= animation_collection
                .current_spritesheet_animation()
                .bounds
                .last_frame_index
        {
            texture_atlas_sprite.index = animation_collection.current_frame_index;
        }
    }
}
