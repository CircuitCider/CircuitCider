use bevy::{
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    reflect::TypePath,
    render::{
        mesh::{MeshVertexBufferLayout, MeshVertexBufferLayoutRef},
        render_resource::{
            AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError,
        },
    },
};

pub const NEON_GLOW_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(13953800272683943019);

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct NeonGlowMaterial {
    #[uniform(0)]
    pub color: LinearRgba,
}

impl From<LinearRgba> for NeonGlowMaterial {
    fn from(color: LinearRgba) -> Self {
        NeonGlowMaterial { color }
    }
}

impl From<NeonGlowMaterial> for LinearRgba {
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
        _layout: &MeshVertexBufferLayoutRef,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.primitive.cull_mode = None;
        Ok(())
    }
}
