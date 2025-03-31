use bevy_asset::Asset;

use bevy_pbr::{MaterialPipeline, MaterialPipelineKey, prelude::*};
use bevy_reflect::Reflect;
use bevy_reflect::prelude::ReflectDefault;
use bevy_render::{
    mesh::MeshVertexBufferLayoutRef,
    render_resource::{
        AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError,
    },
};

const SHADER_PATH: &str = "root://shaders/grid.wgsl";
// const SHADER_PATH: &str = "../../../assets/shaders/grid.wgsl";
// const SHADER_PATH: &str = "shaders/grid.wgsl";

#[derive(Asset, Reflect, AsBindGroup, Debug, Clone, Default)]
#[reflect(Default, Debug)]
pub struct GridMaterial {}

impl Material for GridMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_PATH.into()
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayoutRef,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        // descriptor.primitive.polygon_mode = PolygonMode::Fill;
        // descriptor.primitive.cull_mode = Some(Face::Front);
        // descriptor.primitive.front_face = FrontFace::Ccw;
        if let Some(depth_stencil) = descriptor.depth_stencil.as_mut() {
            depth_stencil.bias.slope_scale = 1.0;
        }
        Ok(())
    }
}
