use std::fmt::{Debug, Formatter};

use bevy::{
    time::{Timer, TimerMode},
    utils::HashMap,
};

use crate::animation::Animation;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum AnimationTransitionMode {
    #[default]
    AfterFinish,
    Immediate,
}

pub type Condition = Box<dyn Fn(&HashMap<String, bool>) -> bool + Send + Sync>;

pub struct AnimationTransitionCondition {
    pub condition: Condition,
    pub mode: AnimationTransitionMode,
}

impl AnimationTransitionCondition {
    pub fn new(condition: Condition) -> Self {
        Self {
            condition,
            mode: AnimationTransitionMode::AfterFinish,
        }
    }

    pub fn with_mode(self, mode: AnimationTransitionMode) -> Self {
        Self { mode, ..self }
    }
}

pub struct AnimationGraphEdge {
    pub(crate) condition: AnimationTransitionCondition,
    pub(crate) neighbour_index: usize,
}

impl Debug for AnimationGraphEdge {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AnimationGraphEdge")
            .field("neighbour_index", &self.neighbour_index)
            .finish()
    }
}

#[derive(Debug)]
pub struct AnimationGraphNode {
    pub(crate) value: Animation,
    pub(crate) edges: Vec<AnimationGraphEdge>,
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

    pub fn add_edge(
        &mut self,
        start_index: usize,
        end_index: usize,
        condition: AnimationTransitionCondition,
    ) {
        self.nodes[start_index].edges.push(AnimationGraphEdge {
            condition,
            neighbour_index: end_index,
        });
    }

    pub fn active_animation(&self) -> &Animation {
        &self.nodes[self.active].value
    }

    pub(crate) fn set_active_node(&mut self, index: usize) {
        println!("setting animation to: {}", index);

        self.active = index;
        self.active_animation_timer =
            Timer::new(self.active_animation().frame_duration, TimerMode::Repeating);
        self.active_animation_finished = false;
    }
}
