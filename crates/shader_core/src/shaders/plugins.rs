use std::default;

use bevy::{asset::load_internal_asset, prelude::*, render::{render_graph::RenderGraphApp, render_phase::AddRenderCommand}};

use crate::shaders::neon::{self, *};

use super::{flow_wireframe::FlowWireframeMaterial, glow::GlowMaterial, ShaderLoadSettings};

/// !!! ADD THIS TO PLUGINS WHEN USING SHADERS FROM THIS MODULE OR BEVY WILL CRASH !!!
pub struct CustomShadersPlugin;

impl Plugin for CustomShadersPlugin {
    fn build(&self, app: &mut App) {

        // load shaders
        app.register_asset_reflect::<GlowMaterial>();
        app.register_asset_reflect::<FlowWireframeMaterial>();
        app.add_plugins(MaterialPlugin::<FlowWireframeMaterial>::default());
        //app.add_plugins(MaterialPlugin::<NeonMaterial>::default());
        app.add_plugins(MaterialPlugin::<GlowMaterial>::default());
    }
}
