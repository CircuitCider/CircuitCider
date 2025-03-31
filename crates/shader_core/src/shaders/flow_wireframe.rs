use bevy_asset::Asset;
use bevy_color::LinearRgba;
use bevy_ecs::component::Component;
use bevy_pbr::{MaterialPipeline, MaterialPipelineKey, prelude::*};
use bevy_reflect::Reflect;
use bevy_reflect::prelude::ReflectDefault;
use bevy_render::{
    mesh::MeshVertexBufferLayoutRef,
    render_resource::{
        AsBindGroup, CompareFunction, PolygonMode, RenderPipelineDescriptor, ShaderRef,
        SpecializedMeshPipelineError,
    },
};

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
#[reflect(Default, Debug)]
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
            //depth_stencil.bias.slope_scale = 1.0;
            depth_stencil.depth_compare = CompareFunction::Never
        }
        Ok(())
    }
}
