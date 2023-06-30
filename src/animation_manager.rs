use bevy::{
    prelude::*,
    utils::{Entry, HashMap},
};

use crate::{
    animation::Animation,
    animation_graph::{
        AnimationGraph, AnimationGraphNode, AnimationTransitionCondition, AnimationTransitionMode,
    },
};

pub struct AnimationManagerPlugin;

impl Plugin for AnimationManagerPlugin {
    fn build(&self, _app: &mut App) {}
}

#[derive(Component, Debug)]
pub struct AnimationManager<T: Animation> {
    // TODO: Add support for different types of state
    state: HashMap<String, bool>,
    graph: AnimationGraph<T>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnimationManagerErr {
    UnknownState { name: String },
}

impl<T: Animation> AnimationManager<T> {
    pub fn new(nodes: Vec<T>, start_node: usize) -> Self {
        let mut graph = AnimationGraph::new(
            nodes
                .into_iter()
                .map(|node| AnimationGraphNode::new(node))
                .collect(),
        );

        assert!(
            start_node < graph.nodes.len(),
            "Start node index is out of bounds"
        );

        graph.set_active_node(start_node);

        Self {
            state: HashMap::new(),
            graph,
        }
    }

    pub fn add_state(&mut self, name: String, value: bool) {
        self.state.insert(name, value);
    }

    pub fn set_state(&mut self, name: String, value: bool) -> Result<(), AnimationManagerErr> {
        match self.state.entry(name.clone()) {
            Entry::Vacant(..) => Err(AnimationManagerErr::UnknownState { name }),
            Entry::Occupied(entry) => Ok(entry),
        }?
        .replace_entry(value);

        Ok(())
    }

    pub fn get_state(&self, name: String) -> Option<bool> {
        self.state.get(&name).map(|state| *state)
    }

    fn is_condition_met(&self, condition: &AnimationTransitionCondition) -> bool {
        (condition.condition)(&self.state)
    }

    pub fn add_graph_edge(
        &mut self,
        start_index: usize,
        end_index: usize,
        condition: AnimationTransitionCondition,
    ) {
        self.graph.add_edge(start_index, end_index, condition);
    }
}

pub fn transition_animations<T: Animation>(
    mut commands: Commands,
    mut query: Query<(Entity, &mut AnimationManager<T>, Option<&T>)>,
) {
    for (entity, mut animation_manager, animation) in query.iter_mut() {
        if animation.is_none() {
            commands
                .entity(entity)
                .insert(animation_manager.graph.active_animation().clone());
            return;
        }

        let animation = animation.unwrap();

        let active_node_index = animation_manager.graph.active;
        let edges = &animation_manager.graph.nodes[active_node_index].edges;

        let next_index = {
            let mut result = None;

            for edge in edges.iter() {
                if (edge.condition.mode == AnimationTransitionMode::Immediate
                    || (edge.condition.mode == AnimationTransitionMode::AfterFinish
                        && animation.finished()))
                    && animation_manager.is_condition_met(&edge.condition)
                {
                    result = Some(edge.neighbour_index);
                    break;
                }
            }

            result
        };

        if let Some(next_index) = next_index {
            animation_manager.graph.set_active_node(next_index);

            commands
                .entity(entity)
                .insert(animation_manager.graph.active_animation().clone());
        }
    }
}
