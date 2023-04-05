use bevy::prelude::*;

use crate::animation::Animation;

#[derive(Debug)]
pub struct AnimationGraphEdge {
    condition: String, 
    neighbour_index: usize,
}

#[derive(Debug)]
pub struct AnimationGraphNode {
    value: Animation,
    edges: Vec<AnimationGraphEdge>
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
    nodes: Vec<AnimationGraphNode>,
    active: usize,
}

impl AnimationGraph {
    pub fn new(nodes: Vec<AnimationGraphNode>) -> Self {
        Self {
            nodes,
            active: 0,
        }
    }

    pub fn add_edge(&mut self, start_index: usize, end_index: usize, condition: String) {
        self.nodes[start_index].edges.push(AnimationGraphEdge { condition, neighbour_index: end_index });
    }
}
