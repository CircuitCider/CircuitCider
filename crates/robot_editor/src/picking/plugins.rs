pub use bevy::prelude::*;
use bevy_mod_outline::OutlinePlugin;



use super::{components::PickSelected, systems::{
    make_models_pickable, pick_self_select_air_deselect, pick_self_select_deselect, toggle_picking_enabled
}, PickMode};

/// picking settings for this project
pub struct CustomPickingPlugin;

impl Plugin for CustomPickingPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .init_state::<PickMode>()
            .add_plugins(MeshPickingPlugin)
            .insert_resource(MeshPickingSettings {
                 require_markers: true,
                ..default()
            })
            .register_type::<PickSelected>()
            .add_plugins(OutlinePlugin)
            //.add_systems(PreUpdate, toggle_picking_enabled)
            //.add_systems(Update, picking_click_effects)
            .add_systems(Update, pick_self_select_deselect.run_if(in_state(PickMode::PickSelfSelectDeselect)))
            .add_systems(Update, pick_self_select_air_deselect.run_if(in_state(PickMode::PickSelfSelectAirDeselect)))
            .add_systems(Update, make_models_pickable);
    }
}
