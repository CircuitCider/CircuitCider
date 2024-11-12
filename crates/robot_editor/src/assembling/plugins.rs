use bevy::prelude::*;

use super::{components::AssemblingTarget, ui::assembler_ui};


/// plugin for assembling robots
pub struct AssemblingPlugin;

impl Plugin for AssemblingPlugin {
    fn build(&self, app: &mut App) {
        app
        // .init_resource::<AssemblingTarget>()
        .add_systems(Update, assembler_ui)
        // .add_systems(Update, transition_to_assembler)
        ;
    }
}