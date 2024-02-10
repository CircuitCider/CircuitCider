use std::{any::TypeId, collections::HashMap};

use bevy::{asset::{AssetContainer, LoadedFolder}, prelude::*, window::PrimaryWindow};
use bevy_egui::EguiContext;
use bevy_mod_picking::backends::raycast::bevy_mod_raycast::{immediate::{Raycast, RaycastSettings, RaycastVisibility}, CursorRay};
use bevy_rapier3d::{geometry::{Collider, Sensor}, plugin::RapierContext, rapier::geometry::CollisionEventFlags};
use bevy_serialization_extras::prelude::{colliders::ColliderFlag, link::StructureFlag};
use async_trait::async_trait;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

use crate::shaders::neon_glow::GizmoMaterial;


use std::fmt::Debug;
pub struct CachePrefabsPlugin;

impl Plugin for CachePrefabsPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(BuildToolMode::PlacerMode)
        .insert_resource(ModelFolder::default())
        // .insert_resource(
        //     ToolModeRegistry::default()
        //     .register(GizmoMode)
        // )
        .add_systems(Startup, cache_initial_folders)
        .add_systems(Update, placer_mode_ui)
        .add_systems(Update, select_build_tool)
        ;
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

pub trait Tool: Send + Sync + Debug{}

#[derive(Resource, Debug)]
pub struct ToolMode {
    pub tool: Box<dyn Tool>
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

#[derive(Resource, Reflect, PartialEq, Eq, EnumIter, Display)]
pub enum BuildToolMode {
    GizmoMode,
    PlacerMode,
}

pub fn select_build_tool(
    mut primary_window: Query<&mut EguiContext, With<PrimaryWindow>>,
    mut tool_mode: ResMut<BuildToolMode>,

) {
    for mut context in primary_window.iter_mut() {
        egui::Window::new("Mesh Selector")
        .show(context.get_mut(), |ui| {
            ui.heading("Spawnable meshes");
            for tool in BuildToolMode::iter() {
                if ui.button(tool.to_string()).clicked() {
                    *tool_mode = tool
                }
            }
        })
        ;
    }
}

pub fn move_placer_to_cursor(
    mut raycast: Raycast, 
    cursor_ray: Res<CursorRay>, 
    //mut transform: Query<&mut Transform>,
    mut placers: Query<(&mut Transform, &Placer)>,
    mut gizmos: Gizmos
) {
    if let Some(cursor_ray) = **cursor_ray {
        // get first raycast hit that isn't the placer it self.
        if let Some((e, hit)) = raycast.debug_cast_ray(cursor_ray, &DONT_EXIT_EARLY, &mut gizmos)
        .iter()
        .filter(|(e, ..)| placers.contains(e.clone()) == false)
        .collect::<Vec<_>>()
        .first()
         {
            for (mut trans, .. ) in placers.iter_mut() {
                let hit_pos = hit.position();
                println!("moving placer to cursor {:#?}", hit_pos);
                trans.translation = hit_pos;
            }
        }

    }
}

/// checks for any intersection between the placer and other meshes
pub fn check_placer_robot_intersections(
    rapier_context: Res<RapierContext>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    placers: Query<(Entity, &Handle<StandardMaterial>, &Placer)>,
) {
    for (e, handle, ..) in placers.iter() {
        if let Some(mat) = materials.get_mut(handle) {
            if rapier_context.intersections_with(e)
            .collect::<Vec<_>>()
            .len() > 0 {
                *mat = Color::RED.into();
            } else {
                *mat = Color::GREEN.into();
            }
        }
    }
}

/// list all placeable models
pub fn placer_mode_ui(
    //mut raycast: Raycast, 
    //cursor_ray: Res<CursorRay>, 
    folders: Res<Assets<LoadedFolder>>,
    model_folder: Res<ModelFolder>,
    tool_mode: Res<BuildToolMode>,
    //meshes: Res<Assets<Mesh>>,
    mut primary_window: Query<&mut EguiContext, With<PrimaryWindow>>,
    mut placer_materials: ResMut<Assets<GizmoMaterial>>,
    mut commands: Commands,

) {
    if tool_mode.into_inner() == &BuildToolMode::PlacerMode {

        let typeid = TypeId::of::<Mesh>();
    
        for mut context in primary_window.iter_mut() {
            egui::SidePanel::right("prefab meshes")
            .show(context.get_mut(), |ui| {
                if let Some(folder) = folders.get(&model_folder.0) {
                    let handles: Vec<Handle<Mesh>> = folder.handles.clone().into_iter()
                            .filter(|handle| handle.type_id() == typeid)
                            .map(|handle| handle.typed::<Mesh>())
                            .collect::<Vec<_>>();

                    for mesh_handle in handles {
                        //let mesh = meshes.get(mesh_handle.clone()).expect("not loaded");
                        if let Some(path) = mesh_handle.path() {
                            let str_path = path.path().to_str().unwrap();
                            if ui.button(str_path).clicked() {
                                //TODO! put raycasting code here
                                commands.spawn(
                                    (
                                        MaterialMeshBundle {
                                            mesh: mesh_handle,
                                            material: placer_materials.add(
                                                GizmoMaterial {
                                                    color:Color::RED.into() 
                                                }
                                            ),
                                            ..default()
                                        },
                                        Placer, 
                                        ColliderFlag::Convex,
                                        Sensor
                                    )
                                )
                                ;
    
                            }
                        }                        
                    }
                } else {
                    ui.label("could not load folder...")
                    ;
                }  
            })
            ;
        }
    }


}
