//! code for display models.
use bevy::prelude::*;

use combat::ui::Entity;

pub mod components;
pub mod plugins;
pub mod systems;


/// entity marked to be displayed
#[derive(Default, Resource, Reflect)]
pub struct DisplayModel(pub Option<Entity>);