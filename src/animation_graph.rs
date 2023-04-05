use bevy::time::{Timer, TimerMode};

use crate::animation::Animation;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum AnimationTransitionMode {
    #[default]
    AfterFinish,
    Immediate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnimationTransitionCondition {
    pub state: Option<String>,
    pub mode: AnimationTransitionMode,
    pub negative: bool,
}

impl AnimationTransitionCondition {
    pub fn new(state: Option<String>) -> Self {
        Self {
            state,
            mode: AnimationTransitionMode::AfterFinish,
            negative: false,
        }
    }

    pub fn with_mode(self, mode: AnimationTransitionMode) -> Self {
        Self {
            mode,
            ..self
        }
    }

    pub fn negative(self) -> Self {
        Self {
            negative: true,
            ..self
        }
    }
}

#[derive(Debug, Clone)]
pub struct AnimationGraphEdge {
    pub(crate) condition: AnimationTransitionCondition, 
    pub(crate) neighbour_index: usize,
}

#[derive(Debug)]
pub struct AnimationGraphNode {
    pub(crate) value: Animation,
    pub(crate) edges: Vec<AnimationGraphEdge>
}

impl AnimationGraphNode {
    pub fn new(animation: Animation) -> Self {
        Self {
            value: animation,
            edges: vec![],
        }
    }
}

#[derive(Debug)]
pub struct AnimationGraph {
    pub(crate) nodes: Vec<AnimationGraphNode>,
    pub(crate) active: usize,
    pub(crate) active_animation_timer: Timer,
    pub(crate) active_animation_finished: bool,
}

impl AnimationGraph {
    pub fn new(nodes: Vec<AnimationGraphNode>) -> Self {
        Self {
            nodes,
            active: 0,
            active_animation_timer: Timer::from_seconds(0., TimerMode::Repeating),
            active_animation_finished: false,
        }
    }

    pub fn add_edge(&mut self, start_index: usize, end_index: usize, condition: AnimationTransitionCondition) {
        self.nodes[start_index].edges.push(AnimationGraphEdge { condition, neighbour_index: end_index });
    }

    pub fn active_animation(&self) -> &Animation {
        &self.nodes[self.active].value
    }

    pub(crate) fn set_active_node(&mut self, index: usize) {
        self.active = index;
        self.active_animation_timer = Timer::new(
            self.active_animation().frame_duration,
            TimerMode::Repeating,
        );
        self.active_animation_finished = false;
    }
}
