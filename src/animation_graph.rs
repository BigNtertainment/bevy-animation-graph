use std::fmt::{Debug, Formatter};

use bevy::utils::HashMap;

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
pub struct AnimationGraphNode<T: Animation> {
    pub(crate) value: T,
    pub(crate) edges: Vec<AnimationGraphEdge>,
}

impl<T: Animation> AnimationGraphNode<T> {
    pub fn new(animation: T) -> Self {
        Self {
            value: animation,
            edges: vec![],
        }
    }
}

#[derive(Debug)]
pub struct AnimationGraph<T: Animation> {
    pub(crate) nodes: Vec<AnimationGraphNode<T>>,
    pub(crate) active: usize,
}

impl<T: Animation> AnimationGraph<T> {
    pub fn new(nodes: Vec<AnimationGraphNode<T>>) -> Self {
        Self {
            nodes,
            active: 0,
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

    pub fn active_animation(&self) -> &T {
        &self.nodes[self.active].value
    }

    pub(crate) fn set_active_node(&mut self, index: usize) {
        self.active = index;
    }
}
