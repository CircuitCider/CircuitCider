pub use bevy::prelude::*;
use bevy_mod_outline::{OutlinePlugin, OutlineVolume};
use bevy_mod_picking::{
    backends::raycast::RaycastBackendSettings, debug::DebugPickingPlugin, highlight::DefaultHighlightingPlugin, picking_core::PickingPluginsSettings, prelude::RaycastBackend, selection::{PickSelection, SelectionPluginSettings}, DefaultPickingPlugins
};
use transform_gizmo_bevy::GizmoTarget;

use crate::resources::BuildToolMode;

use super::systems::{make_models_pickable, picking_click_effects, picking_interaction_effects, toggle_picking_enabled};

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
            // .add_systems(OnEnter(BuildToolMode), systems)
            .add_systems(PreUpdate, toggle_picking_enabled)
            .add_systems(Update, picking_click_effects)
            .add_systems(Update, picking_interaction_effects)
            .add_systems(Update, make_models_pickable)
            ;
    }
}
