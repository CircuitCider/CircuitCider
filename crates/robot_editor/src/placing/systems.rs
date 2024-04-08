use bevy::prelude::*;
use bevy_mod_raycast::{immediate::Raycast, CursorRay};
use bevy_rapier3d::plugin::RapierContext;

use crate::{
    placing::components::AttachCandidate,
    raycast_utils::{
        resources::MouseOverWindow,
        systems::{cursor_ray_hititer, get_first_hit_without_mut},
    },
    resources::BuildToolMode,
    shaders::neon_glow::NeonGlowMaterial,
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
    tool_mode: ResMut<BuildToolMode>,
    placers: Query<Entity, With<Placer>>,
    mut commands: Commands,
) {
    if *tool_mode != BuildToolMode::PlacerMode {
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
    mut neon_materials: ResMut<Assets<NeonGlowMaterial>>,
    placers: Query<(
        Entity,
        &Handle<NeonGlowMaterial>,
        &Handle<Mesh>,
        &Transform,
        &Placer,
    )>,
    mouse: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut tool_mode: ResMut<BuildToolMode>,
    mouse_over_window: Res<MouseOverWindow>,
) {
    if mouse.just_pressed(MouseButton::Left) && **mouse_over_window == false {
        for (e, handle, mesh, trans, ..) in placers.iter() {
            if let Some(mat) = neon_materials.get_mut(handle) {
                if rapier_context
                    .intersection_pairs_with(e)
                    .collect::<Vec<_>>()
                    .len()
                    > 0
                {
                    *mat = Color::RED.into();
                } else {
                    *mat = Color::GREEN.into();
                }
                println!("placing placer..");

                commands.spawn((
                    MaterialMeshBundle {
                        mesh: mesh.clone(),
                        material: handle.clone(),
                        transform: *trans,
                        ..default()
                    },
                    Edited,
                    AttachCandidate,
                ));
                *tool_mode = BuildToolMode::EditerMode;
            }
        }
    }
    if keys.just_pressed(KeyCode::Escape) {
        for (e, ..) in placers.iter() {
            commands.entity(e).despawn_recursive();
        }
    }
}

/// gets rid of placers if current mode is not placermode
pub fn delete_attach_candidates(
    tool_mode: ResMut<BuildToolMode>,
    placers: Query<Entity, With<AttachCandidate>>,
    mut commands: Commands,
) {
    if *tool_mode != BuildToolMode::EditerMode {
        for e in placers.iter() {
            commands.entity(e).despawn()
        }
    }
}

pub fn move_placer_to_cursor(
    mut raycast: Raycast,
    cursor_ray: Res<CursorRay>,
    tool_mode: ResMut<BuildToolMode>,
    mut placers: Query<&mut Transform, With<Placer>>,
    mouse_over_window: Res<MouseOverWindow>,
) {
    // if let Some(mouse_pos) = **cursor_ray {

    // }
    if *tool_mode == BuildToolMode::PlacerMode {
        if let Some((.., hit)) = get_first_hit_without_mut(
            cursor_ray_hititer(&cursor_ray, &mut raycast, &mouse_over_window),
            &mut placers,
        ) {
            for mut trans in placers.iter_mut() {
                //println!("moving placer to cursor");
                let hit_pos = hit.position();
                //println!("moving placer to cursor {:#?}", hit_pos);
                trans.translation = hit_pos;
            }
        }
    }
}
