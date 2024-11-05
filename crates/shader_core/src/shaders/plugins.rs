use bevy::{asset::load_internal_asset, prelude::*};

use crate::shaders::neon::{self, *};

/// !!! ADD THIS TO PLUGINS WHEN USING SHADERS FROM THIS MODULE OR BEVY WILL CRASH !!!
pub struct CustomShadersPlugin;

impl Plugin for CustomShadersPlugin {
    fn build(&self, app: &mut App) {
        // load shaders
        load_internal_asset!(
            app,
            neon::NEON_GLOW_SHADER_HANDLE,
            "../../assets/shaders/neon.wgsl",
            Shader::from_wgsl
        );
        app.add_plugins(MaterialPlugin::<NeonMaterial>::default());
    }
}
