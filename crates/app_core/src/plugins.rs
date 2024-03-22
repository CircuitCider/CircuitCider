use bevy_app::prelude::*;
use bevy_asset::{
    io::{file::FileAssetReader, AssetSource},
    AssetApp, AssetPlugin, AssetServer,
};
use bevy_obj::ObjPlugin;

use crate::{ExecLocation, ROOT};

/// PUT THIS PLUGIN BEFORE [`DefaultPlugins`]!!!
///
///
/// Adds file sources that other crates rely on.
///
/// if your [`{SOURCE}://{asset}.png`] isn't loading, you're probably missing this plugin,
///
/// or, you need to add your asset source to this plugin.
pub struct AppSourcesPlugin{
    pub exec_location: ExecLocation,
}

impl Plugin for AppSourcesPlugin {
    fn build(&self, app: &mut App) {
        let asset_folder_location = match self.exec_location {
            ExecLocation::CRATE => "../../assets",
            ExecLocation::MAIN => "assets",
        };
        //let root = self.root_dir.clone();

        app
            //.add_plugins(AssetPlugin::default())        // .obj file support
            //.add_plugins(ObjPlugin)

            .register_asset_source(
                ROOT,
                AssetSource::build().with_reader(move || Box::new(FileAssetReader::new(asset_folder_location))),
            );
    }
}