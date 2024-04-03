pub use bevy::prelude::*;

use super::systems::deselect_clicked;

/// extra stuff to tailor bevy_mod_picking to this project
pub struct PickingPluginExtras;

impl Plugin for PickingPluginExtras {
    fn build(&self, app: &mut App) {
        app
        //.add_plugins(MultiSelect)
        //.add_systems(Update, deselect_clicked)
        ;
    }
}
