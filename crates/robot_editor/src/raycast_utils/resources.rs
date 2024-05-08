use bevy::{
    ecs::{entity::Entity, system::Resource},
    prelude::{Deref, DerefMut},
    reflect::Reflect,
};
use bevy_mod_raycast::primitives::IntersectionData;

/// weather mouse is over window or not.
#[derive(Resource, Reflect, Deref, DerefMut, Default)]
pub struct MouseOverWindow(bool);

/// collection of things that [`CursorRay`] hit.
/// 
/// TODO: Give hit filter functions to this as an impl. very clunky to use this ATM.
#[derive(Resource, Default, Deref, DerefMut)]
pub struct CursorRayHits (
    pub Vec<(Entity, IntersectionData)>
);