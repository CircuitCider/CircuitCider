use bevy::prelude::*;
use bevy_picking::pointer::PointerInteraction;
use picking_core::components::PickSelected;

use crate::{
    attaching::components::AttachCandidate,
    systems::non_self_hits,
};

use super::components::Placer;

/// gets rid of placers if current mode is not placermode
pub fn delete_placers(
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
    placers: Query<(Entity, Option<&Children>, &Placer)>,
    mouse: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    pointer: Single<&PointerInteraction>,
    valid_attach_targets: Query<&Mesh3d>,
) {
    //let Some(first_hit, ) = pointer.iter().nth(0);
    if mouse.just_released(MouseButton::Left)
        && pointer.iter().nth(0).map(|(a, b)| b.position).is_some()
    {
        for (e, children, placer) in placers.iter() {
            let Some((target, hit)) = non_self_hits(children, &pointer).first() else {
                return;
            };
            // don't attach when over a window.
            if hit.position == None {
                return;
            }
            println!("Attaching: {:#}", e);
            if valid_attach_targets.contains(*target) {
                commands.entity(e).insert((
                    AttachCandidate {
                        attempt_target: Some(*target),
                    },
                    PickSelected(true),
                ));
                commands.entity(e).remove::<Placer>();
            }
        }
    }
    if keys.just_pressed(KeyCode::Escape) {
        for (e, ..) in placers.iter() {
            commands.entity(e).despawn_recursive();
        }
    }
}
