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

/// Different modes for handling how "selecting" stuff works
#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum PickMode {
    
    #[default]
    /// pick to select, pick to deselect.
    PickSelfSelectDeselect,
    /// pick to select, click on air to deselect
    PickSelfSelectAirDeselect,
}


// pub enum PickLock