use bevy::prelude::*;
use bevy_mod_outline::{OutlineBundle, OutlineVolume};
use bevy_mod_picking::{
    focus::PickingInteraction, highlight::PickHighlight, picking_core::Pickable,
    selection::PickSelection, PickableBundle,
};

/// de-select clicked selected things.
pub fn deselect_clicked(mut selectables: Query<(&mut PickSelection, &PickingInteraction)>) {
    for (mut pick_state, pick_interaction) in selectables.iter_mut() {
        if pick_state.is_selected == true && *pick_interaction == PickingInteraction::Pressed {
            pick_state.is_selected = false;
        }
    }
}

pub fn make_models_pickable(
    mut commands: Commands,
    models_query: Query<Entity, (With<Handle<Mesh>>, Without<Pickable>)>,
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
