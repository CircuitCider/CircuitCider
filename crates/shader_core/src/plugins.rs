use bevy_mod_outline::OutlinePlugin;
use bevy_pbr::wireframe::WireframePlugin;

use bevy_app::prelude::*;
use bevy_pbr::MaterialPlugin;
use bevy_toon_material::ToonShaderPlugin;
use crate::resources::{ShadersFolder, WgslCache};
use crate::shaders::flow_wireframe::FlowWireframeMaterial;
use crate::shaders::glow::GlowMaterial;
use crate::shaders::grid::GridMaterial;
use crate::shaders::neon::NeonMaterial;
use crate::systems::spawn_toon_shader_cam;
use crate::*;
pub struct ShaderDebugPlugin;

impl Plugin for ShaderDebugPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<WgslCache>()
        .add_plugins(OutlinePlugin)

        // // ui resources
        // .insert_resource(bind_tree::<WgslInUi>())
        // .insert_resource(bind_tree::<NagaUi>())
        // .insert_resource(bind_tree::<WgslOutUi>())
        // folders
        .add_systems(Startup, add_folder::<ShadersFolder>("shaders".to_owned()))
        .add_systems(Update, update_wgsl_cache)
        .add_systems(Startup, spawn_toon_shader_cam)
        // // uis
        // .add_systems(Update, display_wgslin_info)
        // .add_systems(Update, display_naga_info);
        ;
    }
}

/// core shader features for this project
pub struct ShaderCorePlugin;

impl Plugin for ShaderCorePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(OutlinePlugin)
        .add_plugins(WireframePlugin)
        .add_plugins(ToonShaderPlugin)
        .register_asset_reflect::<NeonMaterial>()
        .register_asset_reflect::<GlowMaterial>()
        .register_asset_reflect::<FlowWireframeMaterial>()
        .add_plugins(MaterialPlugin::<FlowWireframeMaterial>::default())
        .add_plugins(MaterialPlugin::<NeonMaterial>::default())
        .add_plugins(MaterialPlugin::<GlowMaterial>::default())
        .add_plugins(MaterialPlugin::<GridMaterial>::default());
    ;

    }
}
