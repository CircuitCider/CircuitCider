pub use bevy::prelude::*;
use strum_macros::Display;

#[derive(Component, Reflect, Display)]
pub enum Wheel {
    Left,
    Right,
}