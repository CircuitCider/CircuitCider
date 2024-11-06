use std::default;

use bevy::{asset::load_internal_asset, prelude::*};

use crate::shaders::neon::{self, *};

use super::glow::GlowMaterial;

/// !!! ADD THIS TO PLUGINS WHEN USING SHADERS FROM THIS MODULE OR BEVY WILL CRASH !!!
pub struct CustomShadersPlugin;

impl Plugin for CustomShadersPlugin {
    fn build(&self, app: &mut App) {
        // load shaders
        // load_internal_asset!(
        //     app,
        //     neon::NEON_GLOW_SHADER_HANDLE,
        //     "../../assets/shaders/neon.wgsl",
        //     Shader::from_wgsl
        // );
        app.register_type::<GlowMaterial>();
        app.add_plugins(MaterialPlugin::<NeonMaterial>::default());
        app.add_plugins(MaterialPlugin::<GlowMaterial>::default());
    }
}
