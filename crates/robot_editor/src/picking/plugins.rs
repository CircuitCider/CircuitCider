use bevy::app::PluginGroupBuilder;
pub use bevy::prelude::*;
use bevy_mod_outline::{OutlinePlugin, OutlineVolume};

use transform_gizmo_bevy::GizmoTarget;

use crate::resources::BuildToolMode;

use super::systems::{make_models_pickable, picking_click_effects, picking_interaction_effects, toggle_picking_enabled};

/// picking settings for this project
pub struct CustomPickingPlugin;

impl Plugin for CustomPickingPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            // add_plugins(DefaultPickingPlugins.build()
            // .set(
            //     PickingPlugin {
            //         is_enabled: true,
            //         is_focus_enabled
            //     }
            // ))
            .add_plugins(DefaultPickingPlugins)
            .add_plugins(MeshPickingPlugin)
            .insert_resource(MeshPickingSettings {
                require_markers: true,
                ..default()
            })
            .add_plugins(OutlinePlugin)

            // .add_systems(OnEnter(BuildToolMode), systems)
            .add_systems(PreUpdate, toggle_picking_enabled)
            .add_systems(Update, picking_click_effects)
            .add_systems(Update, picking_interaction_effects)
            .add_systems(Update, make_models_pickable)
            ;
    }
}
