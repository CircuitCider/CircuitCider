pub use bevy::prelude::*;
use bevy_mod_outline::{OutlinePlugin, OutlineVolume};
use bevy_mod_picking::{
    backends::raycast::RaycastBackendSettings, debug::DebugPickingPlugin, highlight::DefaultHighlightingPlugin, picking_core::PickingPluginsSettings, prelude::RaycastBackend, selection::{PickSelection, SelectionPluginSettings}, DefaultPickingPlugins
};
use transform_gizmo_bevy::GizmoTarget;

use super::systems::{make_models_pickable, toggle_picking_enabled, update_picking};

/// picking plugin for this project.
pub struct PickingPlugin;

impl Plugin for PickingPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.
            add_plugins(DefaultPickingPlugins.build()
                .disable::<DebugPickingPlugin>()
                .disable::<DefaultHighlightingPlugin>()
            )
            .add_plugins(OutlinePlugin)

            // .insert_resource(RaycastBackendSettings {
            //     require_markers: true,
            //     ..default()
            // })
        
            .insert_resource(SelectionPluginSettings {
                click_nothing_deselect_all: true,
                ..default()
            })
            .add_systems(PreUpdate, toggle_picking_enabled)
            .add_systems(Update, update_picking)
            .add_systems(Update, make_models_pickable)
            ;
    }
}
