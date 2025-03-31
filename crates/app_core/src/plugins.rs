use bevy_app::prelude::*;
use bevy_asset::{
    AssetApp,
    io::{AssetSource, file::FileAssetReader},
};

use crate::ROOT;

/// PUT THIS PLUGIN BEFORE [`DefaultPlugins`]!!!
///
///
/// Adds file sources that other crates rely on.
///
/// if your [`{SOURCE}://{asset}.png`] isn't loading, you're probably missing this plugin,
///
/// or, you need to add your asset source to this plugin.
// pub struct AppSourcesPlugin{
//     pub exec_location: ExecLocation,
// }
pub enum AppSourcesPlugin {
    CRATE,
    MAIN,
}

impl Plugin for AppSourcesPlugin {
    fn build(&self, app: &mut App) {
        let asset_folder_location = match *self {
            Self::CRATE => "../../assets",
            Self::MAIN => "assets",
        };

        //let root = self.root_dir.clone();

        app.register_asset_source(
            ROOT,
            AssetSource::build()
                .with_reader(move || Box::new(FileAssetReader::new(asset_folder_location))),
        );
    }
}

// /// put this `!!!AFTER!!!` [`DefaultPlugins`]
// /// contains default features all apps/crates/test enviorments should have
// pub struct AppSetupPlugin;

// impl Plugin for AppSetupPlugin {
//     fn build(&self, app: &mut App) {

//     }
// }
