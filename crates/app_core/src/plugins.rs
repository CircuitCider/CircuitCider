use bevy::asset::io::{file::FileAssetReader, AssetSource};
pub use bevy::prelude::*;

use crate::ROOT;

/// PUT THIS PLUGIN BEFORE [`DefaultPlugins`]!!!
///
///
/// Adds file sources that other crates rely on.
///
/// if your [`{SOURCE}://{asset}.png`] isn't loading, you're probably missing this plugin,
///
/// or, you need to add your asset source to this plugin.
pub struct AppSourcesPlugin;

impl Plugin for AppSourcesPlugin {
    fn build(&self, app: &mut App) {
        app.register_asset_source(
            ROOT,
            AssetSource::build().with_reader(|| Box::new(FileAssetReader::new("../../assets"))),
        );
    }
}
