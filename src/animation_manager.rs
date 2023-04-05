use bevy::{prelude::*, utils::{HashMap, Entry}};

use crate::{animation::Animation, animation_graph::{AnimationGraph, AnimationGraphNode}};

#[derive(Component, Debug)]
pub struct AnimationManager {
    state: HashMap<String, bool>,
    graph: AnimationGraph,
    current_anim_timer: Timer,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnimationManagerErr {
    UnknownState {
        name: String
    },
}

impl AnimationManager {
    pub fn new(nodes: Vec<Animation>) -> Self {
        Self {
            state: HashMap::new(),
            graph: AnimationGraph::new(nodes.iter().map(|node| AnimationGraphNode::new(*node)).collect()),
            current_anim_timer: Timer::from_seconds(0., TimerMode::Once),
        }
    }

    pub fn add_state(&mut self, name: String, value: bool) {
        self.state.insert(name, value);
    }

    pub fn set_state(&mut self, name: String, value: bool) -> Result<(), AnimationManagerErr> {
        match self.state.entry(name.clone()) {
            Entry::Vacant(..) => Err(AnimationManagerErr::UnknownState { name }),
            Entry::Occupied(entry) => Ok(entry)
        }?.replace_entry(value);

        Ok(())
    }

    pub fn get_state(&self, name: String) -> Option<bool> {
        self.state.get(&name).map(|state| *state)
    }
}

fn perform_animations(mut query: Query<(&mut AnimationManager, &mut TextureAtlasSprite)>, time: Res<Time>) {
    for (mut animation_manager, mut sprite) in query.iter_mut() {
        animation_manager.current_anim_timer.tick(time.delta());

        if animation.timer.just_finished() {
            if sprite.index == animation.bounds.last_frame_index {
                animation.finished = true;
                continue;
            } else {
                sprite.index += 1;
            };
        }
    }
}
