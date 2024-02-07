use std::any::TypeId;

use bevy::{asset::{AssetContainer, LoadedFolder}, prelude::*, window::PrimaryWindow};
use bevy_egui::EguiContext;



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

/// list all placeable models
pub fn list_models(
    folders: Res<Assets<LoadedFolder>>,
    model_folder: Res<ModelFolder>,
    meshes: Res<Assets<Mesh>>,
    mut primary_window: Query<&mut EguiContext, With<PrimaryWindow>>,
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
                    let mesh = meshes.get(mesh_handle.clone()).expect("not loaded");
                    if let Some(path) = mesh_handle.path() {
                        let str_path = path.path().to_str().unwrap();
                        if ui.button(str_path).clicked() {
                            //TODO! put raycasting code here
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
