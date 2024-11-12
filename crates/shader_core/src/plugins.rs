use bevy::prelude::*;

use crate::{resources::{ShadersFolder, WgslCache}};
use crate::*;
pub struct ShaderDebugPlugin;

impl Plugin for ShaderDebugPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
        .init_resource::<WgslCache>()
        // // ui resources
        // .insert_resource(bind_tree::<WgslInUi>())
        // .insert_resource(bind_tree::<NagaUi>())
        // .insert_resource(bind_tree::<WgslOutUi>())
        // folders
        .add_systems(Startup, add_folder::<ShadersFolder>("shaders".to_owned()))
        .add_systems(Update, update_wgsl_cache)
        // // uis
        // .add_systems(Update, display_wgslin_info)
        // .add_systems(Update, display_naga_info);
        ;
    }
}