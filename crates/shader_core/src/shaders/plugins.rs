use bevy::prelude::*;
use bevy_toon_material::ToonShaderPlugin;

use crate::shaders::neon::*;

use super::{flow_wireframe::FlowWireframeMaterial, glow::GlowMaterial, grid::GridMaterial};

/// !!! ADD THIS TO PLUGINS WHEN USING SHADERS FROM THIS MODULE OR BEVY WILL CRASH !!!
pub struct CustomShadersPlugin;

impl Plugin for CustomShadersPlugin {
    fn build(&self, app: &mut App) {
        // load shaders
        app
        .add_plugins(ToonShaderPlugin)
        .register_asset_reflect::<NeonMaterial>()
            .register_asset_reflect::<GlowMaterial>()
            .register_asset_reflect::<FlowWireframeMaterial>()
            .add_plugins(MaterialPlugin::<FlowWireframeMaterial>::default())
            .add_plugins(MaterialPlugin::<NeonMaterial>::default())
            .add_plugins(MaterialPlugin::<GlowMaterial>::default())
            .add_plugins(MaterialPlugin::<GridMaterial>::default());
    }
}
