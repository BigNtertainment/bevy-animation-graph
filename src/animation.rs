use std::time::Duration;

use bevy::prelude::*;

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
pub struct Animation {
    pub active: bool,
    pub atlas: Handle<TextureAtlas>,
    pub bounds: AnimationBounds,
    pub frame_duration: Duration,
}

impl Animation {
    pub fn new(atlas: Handle<TextureAtlas>, bounds: AnimationBounds, frame_duration: Duration) -> Self {
        Self {
            atlas,
            bounds,
            frame_duration,
            active: true,
        }
    }
}
