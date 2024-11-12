use bevy::{pbr::{MaterialPipeline, MaterialPipelineKey}, prelude::*, render::{extract_component::ExtractComponentPlugin, mesh::MeshVertexBufferLayoutRef, render_resource::{AsBindGroup, PolygonMode, RenderPipeline, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError}}};


const SHADER_PATH: &str = "root://shaders/flow_wireframe.wgsl";
// const SHADER_PATH: &str = "../../../assets/shaders/flow_wireframe.wgsl";
// const SHADER_PATH: &str = "shaders/flow_wireframe.wgsl";


// pub struct FlowWireFramePipeline;

// impl Plugin for FlowWireFramePipeline {
//     fn build(&self, app: &mut App) {
//         app.add_plugins(
//             ExtractComponentPlugin::<
//         )
//     }
// }


#[derive(Default, AsBindGroup, Debug, Clone, Asset, Reflect, Component)]
pub struct FlowWireframeMaterial {
    #[uniform(0)]
    pub color: LinearRgba,
}

impl Material for FlowWireframeMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_PATH.into()
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayoutRef,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.primitive.polygon_mode = PolygonMode::Line;
        if let Some(depth_stencil) = descriptor.depth_stencil.as_mut() {
            depth_stencil.bias.slope_scale = 1.0;
        }
        Ok(())
    }
}