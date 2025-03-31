//! code for display models.
use std::ops::Deref;

use bevy::{
    gltf::{GltfMesh, GltfNode, GltfPrimitive},
    prelude::*,
    render::view::RenderLayers,
};
use bevy_serialization_assemble::{
    components::DisassembleStage, gltf::GltfVisualModel, traits::Disassemble,
};

pub mod components;
pub mod plugins;
pub mod systems;

#[derive(PartialEq, Clone)]
pub enum DisplayOption {
    Path(String),
    Handle(Handle<GltfNode>),
}

pub enum GltfNodeLoadError {
    /// for heirarchy reduction, multi node mesh gltfs error
    // MoreThanOneNode,
    // /// for heirarchy reduction. multi scene gltfs error
    // MoreThanOneScene,
    /// for heirarchy reduction. multi mesh gltfs error
    NodeLoading,
    MeshLoading,
    MoreThanOneMesh,
    NoMesh,
}

#[derive(Component)]
pub struct DisplayModelLoading;

// pub struct DisplayRequest(pub DisassembleStage<GltfVisualModel>);

/// entity marked to be displayed
#[derive(Default, Resource)]
pub struct DisplayModel(pub Option<DisplayOption>);

// /// gltf_mesh handle
// pub struct

/// extracts mesh from gltf
/// errors if there is more then one
// pub fn mesh_extract_gltf(gltf: Gltf) -> Result<Handle<GltfMesh>, GltfMeshExtractError>{
//     if gltf.meshes.len() > 1 {
//         return Err(GltfMeshExtractError::MoreThanOneMesh)
//     };
//     let Some(mesh) = gltf.meshes.first() else {
//         return Err(GltfMeshExtractError::NoMesh)
//     };

//     Ok(mesh.clone())
// }

/// extracts useful information from gltf node
pub fn extract_gltf_node(
    handle: &Handle<GltfNode>,
    gltf_meshes: Res<Assets<GltfMesh>>,
    gltf_nodes: Res<Assets<GltfNode>>,
) -> Result<(Transform, GltfMesh), GltfNodeLoadError> {
    let Some(gltf_node) = gltf_nodes.get(handle) else {
        return Err(GltfNodeLoadError::NodeLoading);
    };
    let Some(ref gltf_mesh) = gltf_node.mesh else {
        return Err(GltfNodeLoadError::NoMesh);
    };
    let Some(gltf_mesh) = gltf_meshes.get(&gltf_mesh.clone()) else {
        return Err(GltfNodeLoadError::MeshLoading);
    };
    return Ok((gltf_node.transform, gltf_mesh.clone()));
}
