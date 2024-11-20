use bevy::prelude::*;
use bevy_mod_outline::{OutlineBundle, OutlineVolume};
use bevy_mod_picking::{focus::PickingInteraction, prelude::{PickSelection, Pickable}, PickableBundle};
use bevy_rapier3d::{geometry::Sensor, plugin::RapierContext};
use bevy_serialization_extras::prelude::{colliders::ColliderFlag, link::StructureFlag};
use shader_core::shaders::neon::NeonMaterial;
use transform_gizmo_bevy::GizmoTarget;

use crate::{
    attaching::components::AttachCandidate,
    raycast_utils::resources::{CursorRayHits, MouseOverWindow},
    resources::{BuildToolMode, HullsFolder},
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
            commands.entity(e).despawn()
        }    
    }
}

/// checks for any intersection between the placer and other meshes
pub fn attach_placer(
    placers: Query<(
        Entity,
        &Placer,
    )>,
    mouse: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mouse_over_window: Res<MouseOverWindow>,
    hits: Res<CursorRayHits>,
    robots: Query<&StructureFlag>,
) {
    if mouse.just_released(MouseButton::Left) && **mouse_over_window == false {
        for (e, _) in placers.iter() {
            if let Some((target, ..)) = hits.first_with(&robots) {
                commands.entity(e).insert(
                    AttachCandidate {
                        attempt_target: Some(target)
                    }
                );
            } 
            commands.entity(e).remove::<Placer>();

        }
    }
    if keys.just_pressed(KeyCode::Escape) {
        for (e, ..) in placers.iter() {
            commands.entity(e).despawn_recursive();
        }
    }
}


