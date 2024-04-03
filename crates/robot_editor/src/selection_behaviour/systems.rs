use bevy::prelude::*;
use bevy_mod_picking::{
    focus::PickingInteraction, picking_core::Pickable, selection::PickSelection,
};

/// de-select clicked selected things.
pub fn deselect_clicked(mut selectables: Query<(&mut PickSelection, &PickingInteraction)>) {
    for (mut pick_state, pick_interaction) in selectables.iter_mut() {
        if pick_state.is_selected == true && *pick_interaction == PickingInteraction::Pressed {
            pick_state.is_selected = false;
        }
    }
}
