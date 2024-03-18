use bevy::{
    ecs::system::Resource,
    prelude::{Deref, DerefMut},
    reflect::Reflect,
};

/// weather mouse is over window or not.
#[derive(Resource, Reflect, Deref, DerefMut, Default)]
pub struct MouseOverWindow(bool);
