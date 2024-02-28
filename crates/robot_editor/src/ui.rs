use std::{any::TypeId, collections::HashMap, io::ErrorKind};

use async_trait::async_trait;
use bevy::{
    asset::{AssetContainer, LoadedFolder},
    ecs::query::{QueryData, WorldQuery},
    input::mouse::MouseButtonInput,
    prelude::*,
    reflect::erased_serde::Error,
    window::PrimaryWindow,
};
use bevy_egui::EguiContext;
use bevy_mod_raycast::{
    immediate::{Raycast, RaycastSettings, RaycastVisibility},
    primitives::IntersectionData,
    CursorRay,
};
use bevy_rapier3d::{
    geometry::{Collider, Sensor},
    plugin::RapierContext,
    rapier::geometry::CollisionEventFlags,
};
use bevy_serialization_extras::prelude::{colliders::ColliderFlag, link::StructureFlag};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

use crate::shaders::neon_glow::NeonGlowMaterial;

use std::fmt::Debug;
pub struct CachePrefabsPlugin;

impl Plugin for CachePrefabsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BuildToolMode::PlacerMode)
            .insert_resource(ModelFolder::default())
            .add_systems(Startup, cache_initial_folders)
            .add_systems(Update, placer_mode_ui)
            .add_systems(Update, select_build_tool);
    }
}

#[derive(Resource, Default, Deref)]
pub struct ModelFolder(pub Handle<LoadedFolder>);

pub fn cache_initial_folders(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(ModelFolder(asset_server.load_folder("root://models")));
}

/// entity used to place other similar entities.
#[derive(Component, Default)]
pub struct Placer;

/// gets first raycast hit on entity with select component marker
// pub fn first_hit_with::<T: Component> {

// }

const DONT_EXIT_EARLY: RaycastSettings = RaycastSettings {
    visibility: RaycastVisibility::MustBeVisibleAndInView,
    filter: &|_| true,
    early_exit_test: &|_| false,
};

#[derive(Debug)]
pub struct GizmoMode {}

impl Tool for GizmoMode {}

pub trait Tool: Send + Sync + Debug {}

#[derive(Resource, Debug)]
pub struct ToolMode {
    pub tool: Box<dyn Tool>,
}

/// list of tools that have been registered to select from
// #[derive(Resource, Default)]
// pub struct ToolModeRegistry {
//     pub registered_tools: HashMap<String, Box<dyn Tool>>
// }

// impl ToolModeRegistry {
//     /// registers tool so tool selector widgets know it can be picked.
//     pub fn register<T: Tool + Debug>(&mut self) -> &Self {
//         self.registered_tools.insert(format!("{:#?}", std::any::type_name::<Option<String>>()), &dyn T);
//         self
//     }
// }

#[derive(Resource, Clone, Copy, Reflect, Debug, PartialEq, Eq, EnumIter, Display)]
pub enum BuildToolMode {
    GizmoMode,
    PlacerMode,
    SelectorMode,
    //AttachMode,
    EditerMode,
}

pub fn select_build_tool(
    mut primary_window: Query<&mut EguiContext, With<PrimaryWindow>>,
    mut tool_mode: ResMut<BuildToolMode>,
) {
    for mut context in primary_window.iter_mut() {
        egui::Window::new("BuildToolMode debug").show(context.get_mut(), |ui| {
            ui.heading("select mode");
            ui.label(format!("Current mode: {:#?}", *tool_mode));
            for tool in BuildToolMode::iter() {
                if ui.button(tool.to_string()).clicked() {
                    *tool_mode = tool
                }
            }
        });
    }
}

/// gets first hit with raycast from cursor which doesn't match given query.
pub fn get_first_hit_without<T: QueryData>(
    cursor_ray: Res<CursorRay>,
    mut raycast: Raycast,
    hit_match_criteria: &Query<T>,
) -> Option<(Entity, IntersectionData)> {
    let ray = (**cursor_ray)?;

    let hit_list = raycast
        .cast_ray(ray, &DONT_EXIT_EARLY)
        .iter()
        .filter(|(e, ..)| hit_match_criteria.contains(e.clone()) == false)
        .collect::<Vec<_>>();

    let first_hit = (*hit_list.first()?).clone();
    Some(first_hit)
}

/// gets first hit with raycast from cursor which matches a given query.
pub fn get_first_hit_with<T: QueryData>(
    cursor_ray: Res<CursorRay>,
    mut raycast: Raycast,
    hit_match_criteria: &Query<T>,
) -> Option<(Entity, IntersectionData)> {
    let ray = (**cursor_ray)?;

    let hit_list = raycast
        .cast_ray(ray, &DONT_EXIT_EARLY)
        .iter()
        .filter(|(e, ..)| hit_match_criteria.contains(e.clone()) == true)
        .collect::<Vec<_>>();

    let first_hit = (*hit_list.first()?).clone();
    Some(first_hit)
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
    raycast: Raycast,
    cursor_ray: Res<CursorRay>,
    tool_mode: ResMut<BuildToolMode>,
    mut placers: Query<(&mut Transform, &Placer)>,
) {
    // if let Some(mouse_pos) = **cursor_ray {

    // }
    if *tool_mode == BuildToolMode::PlacerMode {
        if let Some((_, hit)) = get_first_hit_without(cursor_ray, raycast, &placers) {
            for (mut trans, ..) in placers.iter_mut() {
                let hit_pos = hit.position();
                //println!("moving placer to cursor {:#?}", hit_pos);
                trans.translation = hit_pos;
            }
        }
    }
}

pub fn debug_mouse_info(
    cursor_ray: Res<CursorRay>,
    mut raycast: Raycast,
    mut primary_window: Query<&mut EguiContext, With<PrimaryWindow>>,
    mut gizmos: Gizmos
) {
    for mut context in primary_window.iter_mut() {
        egui::Window::new("mouse info")
        .show(context.get_mut(), |ui| {
            ui.label("Mouse ray info");
            if let Some(ray) = **cursor_ray {
                ui.label(format!("{:#?}", ray));
                //gizmos.ray(ray.origin, *ray.direction, Color::RED);
                let orientation = Quat::from_rotation_arc(Vec3::NEG_Z, *ray.direction);
                gizmos.arrow(ray.origin, *ray.direction, Color::RED);
                //gizmos.sphere(ray.origin + *ray.direction, orientation, 0.1, Color::BLUE);
                //raycast.debug_cast_ray(ray, &RaycastSettings::default(), &mut gizmos);
            } 
        });
    }

}

#[derive(Component, Default)]
pub struct Edited;

/// marker for objects that are not yet a part of a structure but could be
/// (placed build mode models)
#[derive(Component, Default)]
pub struct AttachCandidate;

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
    mut commands: Commands,
    mut tool_mode: ResMut<BuildToolMode>,
) {
    if mouse.just_pressed(MouseButton::Left) {
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
}

// /// editor mode for editing attached
// pub fn editor_mode_ui

/// list all placeable models
pub fn placer_mode_ui(
    //mut raycast: Raycast,
    //cursor_ray: Res<CursorRay>,
    folders: Res<Assets<LoadedFolder>>,
    model_folder: Res<ModelFolder>,
    mut tool_mode: ResMut<BuildToolMode>,
    //meshes: Res<Assets<Mesh>>,
    mut primary_window: Query<&mut EguiContext, With<PrimaryWindow>>,
    mut placer_materials: ResMut<Assets<NeonGlowMaterial>>,
    mut commands: Commands,
) {
    //if tool_mode.into_inner() == &BuildToolMode::PlacerMode {

    let typeid = TypeId::of::<Mesh>();

    for mut context in primary_window.iter_mut() {
        egui::SidePanel::right("prefab meshes").show(context.get_mut(), |ui| {
            if let Some(folder) = folders.get(&model_folder.0) {
                let handles: Vec<Handle<Mesh>> = folder
                    .handles
                    .clone()
                    .into_iter()
                    .filter(|handle| handle.type_id() == typeid)
                    .map(|handle| handle.typed::<Mesh>())
                    .collect::<Vec<_>>();

                for mesh_handle in handles {
                    //let mesh = meshes.get(mesh_handle.clone()).expect("not loaded");
                    if let Some(path) = mesh_handle.path() {
                        let str_path = path.path().to_str().unwrap();
                        if ui.button(str_path).clicked() {
                            //TODO! put raycasting code here
                            commands.spawn((
                                MaterialMeshBundle {
                                    mesh: mesh_handle,
                                    material: placer_materials.add(NeonGlowMaterial {
                                        color: Color::RED.into(),
                                    }),
                                    ..default()
                                },
                                Placer,
                                ColliderFlag::Convex,
                                Sensor,
                            ));
                            *tool_mode = BuildToolMode::PlacerMode
                        }
                    }
                }
            } else {
                ui.label("could not load folder...");
            }
        });
    }
    //}
}
