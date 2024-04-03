pub use bevy::prelude::*;
use strum_macros::Display;

#[derive(Component, Reflect, Display)]
pub enum Wheel {
    Left,
    Right,
}

#[derive(Component, Reflect, Default)]
pub struct GizmoFocused;

/// camera that renders models loaded to images for display.
#[derive(Component, Reflect, Default)]
pub struct DisplayModelCamera;
