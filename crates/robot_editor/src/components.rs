pub use bevy::prelude::*;
use strum_macros::Display;

use crate::picking::components::PickSelected;


#[derive(Component, Reflect, Display)]
#[reflect(Component)]
pub enum Wheel {
    Left,
    Right,
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[require(PickSelected)]
pub struct BuildWidgetTarget;

// #[derive(Component, Reflect, Default)]
// pub struct GizmoFocused;
