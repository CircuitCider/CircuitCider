use bevy_derive::{Deref, DerefMut};
use bevy_ecs::prelude::*;
use bevy_picking::prelude::*;
use bevy_reflect::prelude::*;

#[derive(Component, Reflect, Deref, DerefMut)]
#[require(PickingBehavior)]
#[reflect(Component, Default)]
pub struct PickSelected(pub bool);

impl Default for PickSelected {
    fn default() -> Self {
        Self(true)
    }
}

/// Component that marks this entities to take picks up the pick chain. Used for selecting models from primitives of those models.
#[derive(Component, Clone)]
pub struct PickCollector;
