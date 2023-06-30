use bevy::prelude::*;

/// Trait used for implementing your own animation types.
pub trait Animation : Component + Clone {
    fn finished(&self) -> bool;
}
