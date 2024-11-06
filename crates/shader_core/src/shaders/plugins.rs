use std::default;

use bevy::{asset::load_internal_asset, prelude::*};

use crate::shaders::neon::{self, *};

use super::glow::GlowMaterial;

/// !!! ADD THIS TO PLUGINS WHEN USING SHADERS FROM THIS MODULE OR BEVY WILL CRASH !!!
pub struct CustomShadersPlugin;

impl Plugin for CustomShadersPlugin {
    fn build(&self, app: &mut App) {
        // load shaders
        app.register_asset_reflect::<GlowMaterial>();
        app.add_plugins(MaterialPlugin::<NeonMaterial>::default());
        app.add_plugins(MaterialPlugin::<GlowMaterial>::default());
    }
}
