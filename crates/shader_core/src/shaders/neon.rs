use bevy_asset::{Asset, Handle};

use bevy_color::LinearRgba;
use bevy_pbr::{MaterialPipeline, MaterialPipelineKey, prelude::*};
use bevy_reflect::Reflect;
use bevy_render::{
    alpha::AlphaMode,
    mesh::MeshVertexBufferLayoutRef,
    render_resource::{
        AsBindGroup, RenderPipelineDescriptor, Shader, ShaderRef, SpecializedMeshPipelineError,
    },
};

const SHADER_PATH: &str = "root://shaders/neon.wgsl";

pub const NEON_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(13953800272683943019);

#[derive(Asset, AsBindGroup, Debug, Clone, Reflect)]
pub struct NeonMaterial {
    #[uniform(0)]
    pub color: LinearRgba,
}

impl Default for NeonMaterial {
    fn default() -> Self {
        Self {
            color: LinearRgba::WHITE,
        }
    }
}

impl From<LinearRgba> for NeonMaterial {
    fn from(color: LinearRgba) -> Self {
        NeonMaterial { color }
    }
}

impl From<NeonMaterial> for LinearRgba {
    fn from(value: NeonMaterial) -> Self {
        value.color
    }
}

impl Material for NeonMaterial {
    fn vertex_shader() -> ShaderRef {
        SHADER_PATH.into()
        //NEON__SHADER_HANDLE.into()
    }

    fn fragment_shader() -> ShaderRef {
        SHADER_PATH.into()
        //NEON_GLOW_SHADER_HANDLE.into()
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
