
use bevy::{gltf::{GltfMesh, GltfNode}, prelude::*, render::view::RenderLayers};
use bevy_serialization_assemble::{components::{DisassembleAssetRequest, RollDown}, gltf::GltfVisualModel, traits::DisassembleSettings, Assemblies};


use crate::model_display::{extract_gltf_node, DisplayOption};

use super::{components::*, plugins::DISPLAY_MODEL_TRANSLATION, DisplayModel, DisplayModelLoading};

/// enviorment display models are showcased in.
pub fn setup_display_area(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    //camera
    commands.spawn((
        Camera {
            order: 4,
            ..default()
        },
        Camera3d::default(),
        Transform::from_xyz(0.0, 2.5, 4.7).with_rotation(Quat::from_rotation_x(-0.5)),
        RenderLayers::layer(1),
        Name::new("Display Camera"),
    ));

    // plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::new(
            Vec3::new(0.0, 1.0, 0.0),
            Vec2::new(50.0, 50.0),
        ))),
        MeshMaterial3d(materials.add(Color::LinearRgba(LinearRgba::GREEN))),
        Transform::from_xyz(0.0, -47.2, -91.5),
        RenderLayers::layer(1),
        Name::new("Display Floor"),
        DisplayModelStaging,
    ));
}

pub fn stage_display_model(
    display_model: Res<DisplayModel>,
    mut commands: Commands,
    display_models: Query<Entity, With<DisplayRoot>>,
) {
    println!("staging display model");
    if display_model.0.is_some() {
        commands.spawn(
            (
                DisplayRoot,
                Name::new("display model"),
                Transform::from_translation(DISPLAY_MODEL_TRANSLATION),
                RenderLayers::layer(1),
                Visibility::default(),
                DisplayModelLoading,
            )
        );
    } else {
        for e in display_models.iter() {
            commands.entity(e).despawn_recursive();
        }
    }
}

pub fn populate_display_model(
    model: ResMut<DisplayModel>,
    mut display_model: Query<Entity, With<DisplayModelLoading>>,
    mut commands: Commands,
) {
    let Ok(root) = display_model.get_single_mut() else {
        return;
    };
    let Some(kind) = model.0.clone() else {
        return;
    };
    println!("spawning desplay model");
    
    let request = match kind {
        DisplayOption::Path(path) => DisassembleAssetRequest::<GltfVisualModel>::path(path, None),
        DisplayOption::Handle(handle) => DisassembleAssetRequest::handle(handle, None),
    };
    commands.entity(root).insert(
        (
            request,
            RollDown(RenderLayers::layer(1), vec![]),
            
        )
    );
    commands.entity(root).remove::<DisplayModelLoading>();
}

pub fn rotate_display_model(
    mut display_models: Query<&mut Transform, With<DisplayRoot>>,
    time: Res<Time>,
) {
    for mut trans in display_models.iter_mut() {
        //trans.rotate_x(1.0 * time.delta_seconds());
        trans.rotate_y(0.55 * time.delta_secs());
    }
}

pub fn manage_display_platform_visibility(
    display_models: Query<&DisplayRoot>,
    mut display_platforms: Query<&mut Visibility, With<DisplayModelStaging>>,
) {
    if display_models.iter().len() <= 0 {
        for mut vis in display_platforms.iter_mut() {
            *vis = Visibility::Hidden
        }
    } else {
        for mut vis in display_platforms.iter_mut() {
            *vis = Visibility::Inherited
        }
    }
}



// /// helper function for displaying display models
// pub fn display_model(
//     //commands: &'a mut Commands,
//     //neon_materials: &'a mut ResMut<Assets<NeonMaterial>>,
//     //meshes: &'a mut ResMut<Assets<Mesh>>,
//     mut display_model: ResMut<DisplayModel>,
//     root: Entity,
// ) {
//     display_model.0 = Some(root)
//     // commands.spawn((
//     //     //Mesh3d(mesh),
//     //     Transform::from_translation(DISPLAY_MODEL_TRANSLATION),
//     //     MeshMaterial3d(neon_materials.add(LinearRgba::BLUE)),
//     //     RenderLayers::layer(1),
//     //     Name::new("showcase model"),
//     //     DisplayModel,
//     // ));
// }
