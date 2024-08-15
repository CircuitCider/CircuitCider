pub use bevy::prelude::*;
use bevy_mod_outline::{OutlinePlugin, OutlineVolume};
use bevy_mod_picking::{
    debug::DebugPickingPlugin,
    highlight::DefaultHighlightingPlugin,
    picking_core::PickingPluginsSettings,
    selection::{PickSelection, SelectionPluginSettings},
    DefaultPickingPlugins,
};
use transform_gizmo_bevy::GizmoTarget;

use super::systems::{deselect_clicked, make_models_pickable};

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
            .insert_resource(SelectionPluginSettings {
                click_nothing_deselect_all: true,
                ..default()
            })
            .add_systems(PreUpdate, toggle_picking_enabled)
            .add_systems(Update, update_picking)
            .add_systems(Update, make_models_pickable)
            //.add_systems(Update, deselect_clicked)
            ;
    }
}

fn toggle_picking_enabled(
    gizmo_targets: Query<&GizmoTarget>,
    mut picking_settings: ResMut<PickingPluginsSettings>,
) {
    // Picking is disabled when any of the gizmos is focused or active.

    picking_settings.is_enabled = gizmo_targets
        .iter()
        .all(|target| !target.is_focused() && !target.is_active());
}

fn update_picking(
    mut commands: Commands,
    mut targets: Query<(
        Entity,
        &PickSelection,
        &mut OutlineVolume,
        Option<&GizmoTarget>,
    )>,
) {
    // Continuously update entities based on their picking state

    for (entity, pick_interaction, mut outline, gizmo_target) in &mut targets {
        let mut entity_cmd = commands.entity(entity);

        if pick_interaction.is_selected {
            if gizmo_target.is_none() {
                entity_cmd.insert(GizmoTarget::default());
            }

            outline.visible = true;
        } else {
            entity_cmd.remove::<GizmoTarget>();

            outline.visible = false;
        }
    }
}
