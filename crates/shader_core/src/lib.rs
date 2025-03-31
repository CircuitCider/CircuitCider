pub mod plugins;
mod resources;
pub mod shaders;
mod systems;

use std::any::TypeId;

use bevy_asset::{prelude::*, LoadedFolder};
use bevy_ecs::prelude::*;
use bevy_render::render_resource::{Shader, Source};
use resources::{ShadersFolder, WgslCache};

pub struct TreeBehavior {}

#[derive(Default, Debug, Clone)]
pub struct Pane {
    pub name: String,
    pub content: String,
    //pub nr: usize,
}

/// loads assets of type T in a given folder.
pub fn load_assets_for<T: Asset>(
    folders: &Res<Assets<LoadedFolder>>,
    folder_handle: &Handle<LoadedFolder>,
) -> Option<Vec<Handle<T>>> {
    let typeid = TypeId::of::<T>();

    if let Some(folder) = folders.get(folder_handle) {
        let handles: Vec<Handle<T>> = folder
            .handles
            .clone()
            .into_iter()
            .filter(|handle| handle.type_id() == typeid)
            .map(|handle| handle.typed::<T>())
            .collect::<Vec<_>>();
        Some(handles)
    } else {
        None
    }
}

/// update cache of wgsl files that visualizers read from
pub fn update_wgsl_cache(
    folders: Res<Assets<LoadedFolder>>,
    shader_folder_handle: Res<ShadersFolder>,
    mut wgsl_cache: ResMut<WgslCache>,
    shaders: Res<Assets<Shader>>,
) {
    if wgsl_cache.iter().len() <= 0 {
        let Some(shader_handles) = load_assets_for::<Shader>(&folders, &shader_folder_handle)
        else {
            return;
        };

        let shaders = shader_handles
            .iter()
            .filter_map(|handle| shaders.get(handle))
            .collect::<Vec<_>>();

        for shader in shaders.iter() {
            let name = shader
                .path
                .split("/")
                .last()
                .unwrap_or_default()
                .to_string();

            let source_str = match &shader.source {
                Source::Wgsl(str) => str,
                _ => todo!("implement sources other then wgsl later."),
            }
            .to_string();

            wgsl_cache.insert(name, source_str);
        }
    }
}

/// Adds a folder to app by path and binds it to a given newtype struct resource with its handle.
pub fn add_folder<T: From<Handle<LoadedFolder>> + Resource>(
    local_path: String,
) -> impl Fn(&mut World) {
    move |world| {
        let asset_server = world.get_resource_ref::<AssetServer>().unwrap();
        let folder_handle = asset_server.load_folder(local_path.to_owned());
        world.insert_resource(T::from(folder_handle));
    }
}

// pub fn display_wgslout_info(
//     mut primary_window: Query<&mut EguiContext, With<PrimaryWindow>>,
//     mut tree: ResMut<WgslInUi>,
// ) {

// }
