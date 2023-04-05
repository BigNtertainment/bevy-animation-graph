use bevy::{prelude::*, utils::{HashMap, Entry}};

use crate::animation::Animation;

#[derive(Debug)]
pub struct AnimationGraphEdge {
    condition: String, 
    neighbour_index: usize,
}

#[derive(Debug)]
pub struct AnimationGraphNode<'a> {
    value: &'a mut Animation,
    edges: Vec<AnimationGraphEdge>
}

impl<'a> AnimationGraphNode<'a> {
    fn new(animation: &'a mut Animation) -> Self {
        Self {
            value: animation,
            edges: vec![],
        }
    }
}

#[derive(Debug)]
pub struct AnimationGraph<'a> {
    nodes: Vec<AnimationGraphNode<'a>>,
}

impl<'a> AnimationGraph<'a> {
    pub fn new(nodes: Vec<AnimationGraphNode<'a>>) -> Self {
        Self {
            nodes
        }
    }

    pub fn add_edge(&mut self, start_index: usize, end_index: usize, condition: String) {
        self.nodes[start_index].edges.push(AnimationGraphEdge { condition, neighbour_index: end_index });
    }
}

#[derive(Component, Debug)]
pub struct AnimationManager<'a> {
    state: HashMap<String, bool>,
    graph: AnimationGraph<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnimationManagerErr {
    UnknownState {
        name: String
    },
}

impl<'a> AnimationManager<'a> {
    pub fn new(nodes: Vec<&'a mut Animation>) -> Self {
        Self {
            state: HashMap::new(),
            graph: AnimationGraph::new(nodes.iter().map(|node| AnimationGraphNode::new(*node)).collect()),
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