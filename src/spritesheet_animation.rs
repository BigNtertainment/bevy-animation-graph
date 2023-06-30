use std::time::Duration;

use bevy::prelude::*;

use crate::{animation::Animation, animation_app_ext::AnimationAppExt};

pub struct SpritesheetAnimationPlugin;

impl Plugin for SpritesheetAnimationPlugin {
	fn build(&self, app: &mut App) {
		app.register_animation::<SpritesheetAnimation>().add_system(execute_spritesheet_animation);
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

#[derive(Component, Debug, Default, Clone, Reflect)]
pub struct SpritesheetAnimation {
	// TODO: Add support for multiple spritesheets
    bounds: AnimationBounds,
    frame_timer: Timer,
    pub current_frame: usize,
}

impl SpritesheetAnimation {
    pub fn new(
        bounds: AnimationBounds,
        frame_duration: Duration,
    ) -> Self {
        Self {
            bounds,
            frame_timer: Timer::from_seconds(frame_duration.as_secs_f32(), TimerMode::Repeating),
            current_frame: bounds.first_frame_index,
        }
    }
}

impl Animation for SpritesheetAnimation {
    fn finished(&self) -> bool {
        self.current_frame == self.bounds.last_frame_index + 1
    }
}

fn execute_spritesheet_animation(
    mut query: Query<(
        &mut SpritesheetAnimation,
        &mut TextureAtlasSprite,
    )>,
    time: Res<Time>,
) {
    for (mut animation, mut texture_atlas_sprite) in query.iter_mut() {
        if animation.finished() {
            continue;
        }

        animation.frame_timer.tick(time.delta());

        if animation.frame_timer.finished() {
            animation.current_frame += 1;
        }

		if animation.current_frame <= animation.bounds.last_frame_index {
			texture_atlas_sprite.index = animation.current_frame;
		}
    }
}
