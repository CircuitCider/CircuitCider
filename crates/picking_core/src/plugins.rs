
use bevy_app::prelude::*;
use bevy_ecs::schedule::IntoSystemConfigs;
use bevy_picking::mesh_picking::{MeshPickingPlugin, MeshPickingSettings};
use bevy_state::prelude::*;
use bevy_utils::default;

use crate::{components::PickSelected, systems::{pick_self_select_air_deselect, pick_self_select_deselect}, PickMode};


/// picking settings for this project
pub struct CustomPickingPlugin;

impl Plugin for CustomPickingPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<PickMode>()
            .add_plugins(MeshPickingPlugin)
            .insert_resource(MeshPickingSettings {
                 require_markers: true,
                ..default()
            })
            .register_type::<PickSelected>()
            //.add_systems(Update, picking_click_effects)
            .add_systems(Update, pick_self_select_deselect.run_if(in_state(PickMode::PickSelfSelectDeselect)))
            .add_systems(Update, pick_self_select_air_deselect.run_if(in_state(PickMode::PickSelfSelectAirDeselect)))
            ;
    }
}
