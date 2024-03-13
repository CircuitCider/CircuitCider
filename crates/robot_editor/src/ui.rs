use std::{any::TypeId, collections::{HashMap, HashSet}, io::ErrorKind};

use async_trait::async_trait;
use bevy::{
    asset::{AssetContainer, LoadedFolder},
    ecs::query::{QueryData, QueryFilter, ReadOnlyQueryData, WorldQuery},
    input::mouse::MouseButtonInput,
    prelude::*,
    reflect::erased_serde::Error,
    window::PrimaryWindow,
};
use std::hash::Hash;
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
use crate::{raycast_utils::{resources::MouseOverWindow, systems::*}, resources::BuildToolMode};
use crate::shaders::neon_glow::NeonGlowMaterial;

use std::fmt::Debug;


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



// #[derive(Debug)]
// pub struct GizmoMode {}

// impl Tool for GizmoMode {}

// pub trait Tool: Send + Sync + Debug {}

// #[derive(Resource, Debug)]
// pub struct ToolMode {
//     pub tool: Box<dyn Tool>,
// }

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

/// Sets mouse over window resource to true/false depending on mouse state. 
pub fn check_if_mouse_over_ui(
    mut windows: Query<&mut EguiContext>,
    mut mouse_over_window: ResMut<MouseOverWindow>,
) {
    for mut window in windows.iter_mut() {
        if window.get_mut().is_pointer_over_area() {
            //println!("mouse is over window");
            **mouse_over_window = true
        } else {
            **mouse_over_window = false
        }
    }
    //**mouse_over_window = false
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
