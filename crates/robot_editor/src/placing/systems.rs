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
    resources::BuildToolMode,
    ui::Edited,
};

use super::{components::Placer, resources::ModelFolder};

pub fn cache_initial_folders(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(ModelFolder(
        asset_server.load_folder("root://editor_model_parts"),
    ));
}

/// gets rid of placers if current mode is not placermode
pub fn delete_placers(
    tool_mode: ResMut<State<BuildToolMode>>,
    placers: Query<Entity, With<Placer>>,
    mut commands: Commands,
) {
    if **tool_mode != BuildToolMode::PlacerMode {
        for e in placers.iter() {
            commands.entity(e).despawn()
        }
    }
}

/// checks for any intersection between the placer and other meshes
pub fn attach_placer(
    //mut raycast: Raycast,
    //cursor_ray: Res<CursorRay>,
    rapier_context: Res<RapierContext>,
    neon_materials: ResMut<Assets<NeonMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    placers: Query<(
        Entity,
        &Handle<NeonMaterial>,
        &Handle<Mesh>,
        &Transform,
        &Placer,
    )>,
    mouse: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut tool_mode: ResMut<NextState<BuildToolMode>>,
    mouse_over_window: Res<MouseOverWindow>,
    hits: Res<CursorRayHits>,
    robots: Query<&StructureFlag>,
) {
    if mouse.just_pressed(MouseButton::Left) && **mouse_over_window == false {
        for (_, handle, mesh, trans, ..) in placers.iter() {
            if let Some((robot, ..)) = hits.first_with(&robots) {
                println!("clicked robot, switching to attach mode.");
                commands.spawn((
                    MaterialMeshBundle {
                        mesh: mesh.clone(),
                        material: handle.clone(),
                        transform: *trans,
                        ..default()
                    },
                    Edited,
                    AttachCandidate {attempt_target: Some(robot) },
                    ColliderFlag::Convex,
                    Sensor,
                    Pickable::default(),
                    PickSelection {
                        is_selected: true
                    },
                    GizmoTarget::default(),
                    Name::new("Attach Candidate"),
                ));
                tool_mode.set(BuildToolMode::EditerMode);
            } 
            else {
                println!("placing placer..");
                commands.spawn((
                    PbrBundle {
                        mesh: mesh.clone(),
                        material: materials.add(Color::WHITE),
                        transform: *trans,
                        ..default()
                    },
                    Edited,
                    ColliderFlag::Convex,
                    Sensor,
                    Pickable::default(),
                    PickSelection {
                        is_selected: true
                    },
                    GizmoTarget::default(),
                    Name::new("Attach Candidate"),
                ));
                tool_mode.set(BuildToolMode::EditerMode);
            }

        }
    }
    if keys.just_pressed(KeyCode::Escape) {
        for (e, ..) in placers.iter() {
            commands.entity(e).despawn_recursive();
        }
    }
}


