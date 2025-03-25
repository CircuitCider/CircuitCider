use bevy::prelude::*;
use bevy_picking::pointer::PointerInteraction;

use crate::{
    attaching::components::AttachCandidate,
    // raycast_utils::resources::{MouseOverWindow},
    resources::BuildToolMode,
};

use super::components::Placer;

/// gets rid of placers if current mode is not placermode
pub fn delete_placers(
    tool_mode: ResMut<State<BuildToolMode>>,
    placers: Query<Entity, With<Placer>>,
    mut commands: Commands,
    keys: Res<ButtonInput<MouseButton>>,
) {
    let mut despawn = false;
    // if **tool_mode != BuildToolMode::PlacerMode {
    //     despawn = true;
    // }
    if keys.pressed(MouseButton::Right) {
        despawn = true;
    }
    if despawn == true {
        for e in placers.iter() {
            commands.entity(e).despawn_recursive()
        }
    }
}

/// checks for any intersection between the placer and other meshes
pub fn attach_placer(
    placers: Query<(Entity, &Placer)>,
    mouse: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    pointer: Single<&PointerInteraction>,
    // mouse_over_window: Res<MouseOverWindow>,
    // hits: Res<CursorRayHits>,
    //robots: Query<&StructureFlag>,
) {
    //let Some(first_hit, ) = pointer.iter().nth(0);
    if mouse.just_released(MouseButton::Left) && pointer.iter().nth(0).map(|(a, b)| b.position).is_some() {
        for (e, _) in placers.iter() {
            
            // if let Some((target, ..)) = hits.first_with(&robots) {
            //     commands.entity(e).insert(AttachCandidate {
            //         attempt_target: Some(target),
            //     });
            // }
            commands.entity(e).remove::<Placer>();
        }
    }
    if keys.just_pressed(KeyCode::Escape) {
        for (e, ..) in placers.iter() {
            commands.entity(e).despawn_recursive();
        }
    }
}
