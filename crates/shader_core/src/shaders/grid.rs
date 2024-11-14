use bevy::{
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    reflect::TypePath,
    render::{
        mesh::{MeshVertexBufferLayoutRef, PrimitiveTopology},
        render_resource::{
            AsBindGroup, Face, FrontFace, PolygonMode, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError
        },
    },
};



// const SHADER_PATH: &str = "root://shaders/grid.wgsl";
// const SHADER_PATH: &str = "../../../assets/shaders/grid.wgsl";
const SHADER_PATH: &str = "shaders/grid.wgsl";


#[derive(Asset, Reflect, AsBindGroup, Debug, Clone, Default)]
#[reflect(Default, Debug)]
pub struct GridMaterial {
}


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