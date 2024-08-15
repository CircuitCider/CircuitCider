use bevy::{
    ecs::component::Component,
    prelude::{Deref, DerefMut},
};

/// Health points.
#[derive(Component)]
pub struct Health {
    pub hp: i32,
    pub max_hp: i32,
}

impl Default for Health {
    fn default() -> Self {
        Self {
            hp: 100,
            max_hp: 100,
        }
    }
}
