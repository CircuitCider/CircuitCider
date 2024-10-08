use bevy::prelude::*;
use bevy_mod_outline::{OutlineBundle, OutlineVolume};
use bevy_mod_picking::{
    focus::PickingInteraction, highlight::PickHighlight, picking_core::{Pickable, PickingPluginsSettings},
    selection::PickSelection, PickableBundle,
};
use bevy_serialization_extras::prelude::link::StructureFlag;
use transform_gizmo_bevy::GizmoTarget;

/// de-select clicked selected things.
pub fn deselect_clicked(mut selectables: Query<(&mut PickSelection, &PickingInteraction)>) {
    for (mut pick_state, pick_interaction) in selectables.iter_mut() {
        if pick_state.is_selected == true && *pick_interaction == PickingInteraction::Pressed {
            pick_state.is_selected = false;
        }
    }
}

pub fn toggle_picking_enabled(
    gizmo_targets: Query<&GizmoTarget>,
    mut picking_settings: ResMut<PickingPluginsSettings>,
) {
    // Picking is disabled when any of the gizmos is focused or active.

    picking_settings.is_enabled = gizmo_targets
        .iter()
        .all(|target| !target.is_focused() && !target.is_active());
}

pub fn update_picking(
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


pub fn make_models_pickable(
    mut commands: Commands,
    models_query: Query<Entity, (With<StructureFlag>, Without<Pickable>)>,
) {
    for e in models_query.iter() {
        commands.entity(e).insert((
            PickableBundle {
                pickable: Pickable::default(),
                interaction: PickingInteraction::default(),
                selection: PickSelection::default(),
                highlight: PickHighlight::default(),
            },
            OutlineBundle {
                outline: OutlineVolume {
                    visible: false,
                    colour: Color::WHITE,
                    width: 2.0,
                },
                ..default()
            },
        ));
    }
}
