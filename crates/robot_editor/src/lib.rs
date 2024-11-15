use bevy::{color::{Color, LinearRgba}, prelude::Entity};
use bevy_mod_outline::OutlineVolume;

pub mod attaching;
pub mod camera_controls;
pub mod components;
pub mod model_display;
pub mod picking;
pub mod placing;
pub mod plugins;
pub mod raycast_utils;
pub mod resources;
pub mod states;
pub mod systems;
pub mod ui;
pub mod assembling;

pub mod prelude {
    pub use crate::*;
}

/// marks component as targeting something. Used for generic systems.
pub trait Targeter {
    /// what this component targets
    fn targets(&self) -> Option<Entity>;
}

pub enum SpacingKind {
    Uplift(f32),
    None,
}

/// spacing for moving to mouse
pub trait Spacing {
    fn spacing() -> SpacingKind;
}

const ERROR_COLOR: Color = Color::LinearRgba(LinearRgba::new(128.0, 0.0, 128.0, 1.0));

const NO_OUTLINE: OutlineVolume = OutlineVolume {
    visible: false, 
    width: 1.0,
    colour: ERROR_COLOR
};
