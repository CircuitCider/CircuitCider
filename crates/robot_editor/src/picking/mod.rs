//! code for the picking behaviour of the editor.

use bevy::prelude::*;

pub mod plugins;
mod systems;


#[derive(Component, Clone, Copy)]
pub struct PickSelection {
    pub is_selected: bool,
}