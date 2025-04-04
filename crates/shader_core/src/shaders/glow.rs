use bevy_asset::Asset;

use bevy_pbr::{MaterialPipeline, MaterialPipelineKey, prelude::*};
use bevy_reflect::Reflect;
use bevy_reflect::prelude::ReflectDefault;
use bevy_render::{
    alpha::AlphaMode,
    mesh::MeshVertexBufferLayoutRef,
    render_resource::{
        AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError,
    },
};

const SHADER_PATH: &str = "root://shaders/glow.wgsl";
// const SHADER_PATH: &str = "../../../assets/shaders/glow.wgsl";

#[derive(Asset, Reflect, AsBindGroup, Debug, Clone, Default)]
#[reflect(Default, Debug)]
pub struct GlowMaterial {
    #[uniform(0)]
    pub heat: f32,
}

impl Material for GlowMaterial {
    fn vertex_shader() -> ShaderRef {
        SHADER_PATH.into()
    }

    fn fragment_shader() -> ShaderRef {
        SHADER_PATH.into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Opaque
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayoutRef,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.primitive.cull_mode = None;
        Ok(())
    }
}
