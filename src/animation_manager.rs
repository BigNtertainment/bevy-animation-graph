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
    fn build(&self, app: &mut App) {
        app.add_systems((perform_animations, transition_animations).chain());
    }
}

#[derive(Component, Debug)]
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
    pub fn new(nodes: Vec<Animation>, start_node: usize) -> Self {
        let mut graph = AnimationGraph::new(
            nodes
                .into_iter()
                .map(|node| AnimationGraphNode::new(node))
                .collect(),
        );

        assert!(start_node < graph.nodes.len(), "Start node index is out of bounds");

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
        if condition.mode == AnimationTransitionMode::AfterFinish {
            if !self.graph.active_animation_finished {
                return false;
            }
        }

        if let Some(state) = &condition.state {
            match self.state.get(state) {
                Some(state) => {
                    if condition.negative {
                        !state
                    } else {
                        *state
                    }
                }
                None => false,
            }
        } else {
            !condition.negative
        }
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

fn perform_animations(
    mut query: Query<(&mut AnimationManager, &mut TextureAtlasSprite)>,
    time: Res<Time>,
) {
    // TODO: Handling animations using different texture atlases

    for (mut animation_manager, mut sprite) in query.iter_mut() {
        if animation_manager.graph.active_animation_finished {
            continue;
        }

        animation_manager
            .graph
            .active_animation_timer
            .tick(time.delta());

        if animation_manager
            .graph
            .active_animation_timer
            .just_finished()
        {
            if sprite.index
                == animation_manager
                    .graph
                    .active_animation()
                    .bounds
                    .last_frame_index
            {
                animation_manager.graph.active_animation_finished = true;
                continue;
            } else {
                sprite.index += 1;
            };
        }
    }
}

fn transition_animations(mut query: Query<(&mut AnimationManager, &mut TextureAtlasSprite)>) {
    for (mut animation_manager, mut sprite) in query.iter_mut() {
        let active_node_index = animation_manager.graph.active;
        let edges = &animation_manager.graph.nodes[active_node_index]
            .edges;

        let next_index = {
            let mut result = None;

            for edge in edges.iter() {
                if animation_manager.is_condition_met(&edge.condition) {
                    result = Some(edge.neighbour_index);
                    break;
                }
            }

            result
        };

        if let Some(next_index) = next_index {
            animation_manager
                .graph
                .set_active_node(next_index);
            
            sprite.index = animation_manager
                .graph
                .active_animation()
                .bounds
                .first_frame_index;
        }
    }
}
