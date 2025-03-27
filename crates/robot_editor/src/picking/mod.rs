//! code for the picking behaviour of the editor.

use bevy::prelude::*;

pub mod plugins;
pub mod components;
mod systems;


pub const DONT_EXIT_EARLY: RayCastSettings<'static> = RayCastSettings {
    visibility: RayCastVisibility::VisibleInView,
    filter: &|_| true,
    early_exit_test: &|_| false,
};

pub const EXIT_EARLY: RayCastSettings = RayCastSettings {
    visibility: RayCastVisibility::Any,
    filter: &|_| true,
    early_exit_test: &|_| false,
};