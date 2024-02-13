use bevy::{
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    reflect::TypePath,
    render::{
        mesh::MeshVertexBufferLayout,
        render_resource::{
            AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError,
        },
    },
};
use bevy_serialization_extras::prelude::material::MaterialFlag;

pub const NEON_GLOW_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(13953800272683943019);

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct NeonGlowMaterial {
    #[uniform(0)]
    pub color: Color,
}

impl From<Color> for NeonGlowMaterial {
    fn from(color: Color) -> Self {
        NeonGlowMaterial { color }
    }
}

impl From<NeonGlowMaterial> for Color {
    fn from(value: NeonGlowMaterial) -> Self {
        value.color
    }
}

impl Material for NeonGlowMaterial {
    fn vertex_shader() -> ShaderRef {
        NEON_GLOW_SHADER_HANDLE.into()
    }

    fn fragment_shader() -> ShaderRef {
        NEON_GLOW_SHADER_HANDLE.into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Opaque
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.primitive.cull_mode = None;
        Ok(())
    }
}