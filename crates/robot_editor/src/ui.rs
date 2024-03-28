use std::{
    any::TypeId,
    collections::{HashMap, HashSet},
    io::ErrorKind, thread::spawn,
};

use crate::{components::DisplayModelCamera, shaders::neon_glow::NeonGlowMaterial};
use crate::{
    raycast_utils::{resources::MouseOverWindow, systems::*},
    resources::BuildToolMode,
};
use bevy::{
    asset::{AssetContainer, LoadedFolder}, ecs::query::{QueryData, QueryFilter, ReadOnlyQueryData, WorldQuery}, input::mouse::MouseButtonInput, log::tracing_subscriber::field::display, prelude::*, reflect::erased_serde::Error, render::{render_asset::RenderAssets, view::RenderLayers}, window::PrimaryWindow
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
use egui::Align2;
use std::hash::Hash;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

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



#[derive(Component, Default)]
pub struct Edited;

/// marker for objects that are not yet a part of a structure but could be
/// (placed build mode models)
#[derive(Component, Default)]
pub struct AttachCandidate;



// /// editor mode for editing attached
// pub fn editor_mode_ui

pub fn save_load_model_ui(
    mut primary_window: Query<&mut EguiContext, With<PrimaryWindow>>,
    //mut commands: Commands,

) {
    for mut context in primary_window.iter_mut() {
        let ui_name = "Save Load Model";
        egui::Window::new(ui_name)
        .anchor(Align2::RIGHT_TOP, [0.0, 0.0])
        .collapsible(false)
        .resizable(false)
        .show(context.get_mut(), |ui| {    
            ui.label("save conditions");

            ui.horizontal(|ui| {
                ui.button("save");
                //ui.button("load");
            });

        });
        
    }
}

/// model only rendered for display
#[derive(Component)]
pub struct DisplayModel;

#[derive(Resource, Deref, Default)]
pub struct DisplayModelImage(pub Handle<Image>);

/// save the display model image to file so egui can load it
pub fn display_model_image_to_file(
    display_model_image: Res<DisplayModelImage>,
    images: ResMut<Assets<Image>>,
    model_folder: Res<ModelFolder>,
    folders: Res<Assets<LoadedFolder>>,

) {
    let image_handle = &**display_model_image;
    let Some(image) = images.get(image_handle) else {return};
    let Ok(dyn_image) = image.clone().try_into_dynamic() else { return};

    // let typeid = TypeId::of::<Mesh>();

    // let Some(folder) = folders.get(&**model_folder) else {return};
    // let Some(first_item) = folder
    // .handles
    // .clone()
    // .into_iter()
    // .next()
    // else {return};

    // let Some(first_item_path) = first_item.path() else {return};
}

/// list all placeable models
pub fn placer_mode_ui(
    //mut raycast: Raycast,
    //cursor_ray: Res<CursorRay>,
    folders: Res<Assets<LoadedFolder>>,
    model_folder: Res<ModelFolder>,
    mut tool_mode: ResMut<BuildToolMode>,
    //meshes: Res<Assets<Mesh>>,
    mut placer_materials: ResMut<Assets<NeonGlowMaterial>>,
    mut primary_window: Query<&mut EguiContext, With<PrimaryWindow>>,
    display_models: Query<(Entity, &Handle<Mesh>), With<DisplayModel>>,


    mut commands: Commands,
) {
    //if tool_mode.into_inner() == &BuildToolMode::PlacerMode {

    let typeid = TypeId::of::<Mesh>();
    //println!("PREPARING TO ADD STUFF TO PLACE MODE UI");
    //info!("PRIMARY WINDOW COUNT: {:#?}", primary_window.iter().len());
    for mut context in primary_window.iter_mut() {
        //println!("POPULATiNG PLACER MODE UI");
        let ui_name = "prefab meshes";
        egui::SidePanel::left(ui_name).show(context.get_mut(), |ui| {
            ui.heading(ui_name);
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
                        let spawn_button = ui.button(str_path);
                        
                        if spawn_button.clicked() {
                            //TODO! put raycasting code here
                            commands.spawn((
                                MaterialMeshBundle {
                                    mesh: mesh_handle.clone(),
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
                        //spawn display model for hovered over spawnables
                        if spawn_button.hovered() {
                            ui.label("show display model here!");
                            for (e, display_handle) in display_models.iter() {
                                if mesh_handle.path() != display_handle.path() {
                                    commands.entity(e).despawn()
                                }
                            }
                            if display_models.iter().len() < 1 {
                                commands.spawn((
                                    MaterialMeshBundle {
                                        mesh: mesh_handle.clone(),
                                        material: placer_materials.add(NeonGlowMaterial {
                                            color: Color::BLUE.into(),
                                        }),
                                        ..default()
                                    },
                                    DisplayModel,
                                    RenderLayers::layer(1)
                                ));
                            }
                            //ui.image(source)
                        } else {
                            for (e, ..) in display_models.iter() {
                                    commands.entity(e).despawn()
                            }
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
