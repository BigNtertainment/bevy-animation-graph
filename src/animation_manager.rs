use bevy::{
    prelude::*,
    utils::{Entry, HashMap},
};

use crate::{
    animation_collection::AnimationCollection,
    animation_graph::{AnimationGraph, AnimationTransitionCondition, AnimationTransitionMode},
};

pub struct AnimationManagerPlugin;

impl Plugin for AnimationManagerPlugin {
    fn build(&self, _app: &mut App) {}
}

#[derive(Component, Default, Debug)]
pub struct AnimationManager {
    // TODO: Add support for different types of state
    state: HashMap<String, bool>,
    graph: AnimationGraph,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnimationManagerErr {
    UnknownState { name: String },
}

impl AnimationManager {
    pub fn new(node_num: usize, start_node: usize) -> Self {
        let mut graph = AnimationGraph::new(node_num);

        assert!(
            start_node < node_num,
            "start node index is out of bounds"
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

pub fn transition_animations<T: AnimationCollection>(
    mut query: Query<(&mut AnimationManager, &mut T)>,
) {
    for (mut animation_manager, mut animation_collection) in query.iter_mut() {
        let active_node_index = animation_manager.graph.active;
        let edges = &animation_manager.graph.nodes[active_node_index].edges;

        let next_index = {
            let mut result = None;

            for edge in edges.iter() {
                if (edge.condition.mode == AnimationTransitionMode::Immediate
                    || (edge.condition.mode == AnimationTransitionMode::AfterFinish
                        && animation_collection.is_current_animation_finished()))
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

            animation_collection.transition(animation_manager.graph.active);
        }
    }
}
