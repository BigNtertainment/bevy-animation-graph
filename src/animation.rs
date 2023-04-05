use std::time::Duration;

use bevy::prelude::*;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Animation>()
            .add_system(perform_animations);
    }
}

#[derive(Debug, Default, Clone, Copy, Reflect)]
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

#[derive(Component, Debug, Default, Clone, Reflect)]
#[reflect(Component)]
pub struct Animation {
    pub atlas: Handle<TextureAtlas>,
    pub bounds: AnimationBounds,
    timer: Timer,
}

impl Animation {
    pub fn new(atlas: Handle<TextureAtlas>, bounds: AnimationBounds, frame_duration: Duration) -> Self {
        Self {
            atlas,
            bounds,
            timer: Timer::new(frame_duration, TimerMode::Repeating),
        }
    }
}

fn perform_animations(mut query: Query<(&mut Animation, &mut TextureAtlasSprite)>, time: Res<Time>) {
    for (mut animation, mut sprite) in query.iter_mut() {
        animation.timer.tick(time.delta());

        if animation.timer.just_finished() {
            sprite.index = if sprite.index == animation.bounds.last_frame_index {
                animation.bounds.first_frame_index
            } else {
                sprite.index + 1
            };
        }
    }
}
