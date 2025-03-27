use bevy::{ecs::component::*, prelude::{Deref, DerefMut}, reflect::Reflect};



#[derive(Component, Reflect, Deref, DerefMut)]
pub struct PickSelected(pub bool);

impl Default for PickSelected {
    fn default() -> Self {
        Self(true)
    }
}

/// Component that marks this entities to take picks up the pick chain. Used for selecting models from primitives of those models.
#[derive(Component, Clone)]
pub struct PickCollector;