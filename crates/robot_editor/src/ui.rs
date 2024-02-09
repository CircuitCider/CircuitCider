use std::any::TypeId;

use bevy::{asset::{AssetContainer, LoadedFolder}, prelude::*, window::PrimaryWindow};
use bevy_egui::EguiContext;
use bevy_mod_picking::backends::raycast::bevy_mod_raycast::{immediate::{Raycast, RaycastSettings, RaycastVisibility}, CursorRay};
use bevy_rapier3d::geometry::Collider;
use bevy_serialization_extras::prelude::colliders::ColliderFlag;



pub struct CachePrefabsPlugin;

impl Plugin for CachePrefabsPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(ModelFolder::default())
        .add_systems(Startup, cache_initial_folders)
        .add_systems(Update, list_models)
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
        // for (e, hit ) in raycast.cast_ray(cursor_ray, &default()).iter() {

        // }
    }
}

/// checks for any intersection between the placer and other meshes
pub fn check_placer_intersections(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut placers: Query<(Entity, &Handle<StandardMaterial>, &Placer)>,
) {
    for (e, handle, ..) in placers.iter() {
        if let Some(mat) = materials.get_mut(handle) {
            
        }
    }
}


/// list all placeable models
pub fn list_models(
    //mut raycast: Raycast, 
    //cursor_ray: Res<CursorRay>, 
    folders: Res<Assets<LoadedFolder>>,
    model_folder: Res<ModelFolder>,
    //meshes: Res<Assets<Mesh>>,
    mut primary_window: Query<&mut EguiContext, With<PrimaryWindow>>,
    mut commands: Commands,
) {
    let typeid = TypeId::of::<Mesh>();
    for mut context in primary_window.iter_mut() {
        egui::Window::new("prefab meshes")
        .show(context.get_mut(), |ui| {
            if let Some(folder) = folders.get(&model_folder.0) {
                let handles: Vec<Handle<Mesh>> = folder.handles.clone().into_iter()
                        .filter(|handle| handle.type_id() == typeid)
                        .map(|handle| handle.typed::<Mesh>())
                        .collect::<Vec<_>>();
                //println!("displaying contents of folder");
                //println!("mesh count: {:#?}", handles.len());
                for mesh_handle in handles {
                    //let mesh = meshes.get(mesh_handle.clone()).expect("not loaded");
                    if let Some(path) = mesh_handle.path() {
                        let str_path = path.path().to_str().unwrap();
                        if ui.button(str_path).clicked() {
                            //TODO! put raycasting code here
                            commands.spawn(
                                (
                                    PbrBundle {
                                        mesh: mesh_handle,
                                        ..default()
                                        //transform: Transform::from_xyz(0.0, 0.0, 0.0)
                                    },
                                    Placer, 
                                    ColliderFlag::Convex,
                                )
                            )
                            ;

                        }
                    }
                    
                    //println!("displaying mesh {:#?}", mesh);
                }
            } else {
                ui.label("could not load folder...")
                ;
            }  
        });
    }

}
