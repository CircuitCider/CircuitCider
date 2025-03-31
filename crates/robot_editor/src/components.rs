pub use bevy::prelude::*;
use picking_core::components::PickSelected;
use strum_macros::Display;



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


#[derive(Component)]
pub struct PointerFollowTarget;

// #[derive(Component, Reflect, Default)]
// pub struct GizmoFocused;
