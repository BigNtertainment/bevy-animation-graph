use std::fmt::{Debug, Formatter};

use bevy::utils::HashMap;

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

#[derive(Debug, Default)]
pub struct AnimationGraphNode {
    pub(crate) edges: Vec<AnimationGraphEdge>,
}

impl AnimationGraphNode {
    pub fn new() -> Self {
        Self {
            edges: vec![],
        }
    }
}

#[derive(Debug, Default)]
pub struct AnimationGraph {
    pub(crate) nodes: Vec<AnimationGraphNode>,
    pub(crate) active: usize,
}

impl AnimationGraph {
    pub fn new(node_num: usize) -> Self {
        Self {
            nodes: {
                let mut nodes = Vec::with_capacity(node_num);
                for _ in 0..node_num {
                    nodes.push(AnimationGraphNode::new());
                }
                nodes
            },
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

    pub(crate) fn set_active_node(&mut self, index: usize) {
        self.active = index;
    }
}
