
use bevy::{prelude::*, render::view::RenderLayers};
use shader_core::shaders::neon::NeonMaterial;

use super::{components::*, plugins::DISPLAY_MODEL_TRANSLATION};

/// enviorment display models are showcased in.
pub fn setup_display_area(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    //camera
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                order: 4,
                //target: image_handle.clone().into(),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 2.5, 4.7)
                .with_rotation(Quat::from_rotation_x(-0.5)),
            ..default()
        },
        RenderLayers::layer(1),
        Name::new("Display Camera"),
    ));

    // plane
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::new(
                Vec3::new(0.0, 1.0, 0.0),
                Vec2::new(50.0, 50.0),
            )),
            material: materials.add(Color::WHITE),
            transform: Transform::from_xyz(0.0, -47.2, -91.5),
            ..default()
        },
        RenderLayers::layer(1),
        Name::new("Display Floor"),
        DisplayModelStaging,
    ));
}

pub fn rotate_display_model(
    mut display_models: Query<&mut Transform, With<DisplayModel>>,
    time: Res<Time>,
) {
    for mut trans in display_models.iter_mut() {
        //trans.rotate_x(1.0 * time.delta_seconds());
        trans.rotate_y(0.55 * time.delta_seconds());
    }
}

pub fn manage_display_platform_visibility(
    display_models: Query<&DisplayModel>,
    mut display_platforms: Query<&mut Visibility, With<DisplayModelStaging>>,
    commands: Commands,
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

/// helper function for displaying display models
pub fn display_model<'a>(
    commands: &'a mut Commands,
    neon_materials: &'a mut ResMut<Assets<NeonMaterial>>,
    //meshes: &'a mut ResMut<Assets<Mesh>>,
    mesh: Handle<Mesh>,
) {
    // Cube
    commands.spawn((
        MaterialMeshBundle {
            mesh: mesh,
            transform: Transform::from_translation(DISPLAY_MODEL_TRANSLATION)
                .with_rotation(Quat::from_euler(EulerRot::XYZ, 0.0, 0.7, 0.0)),
            material: neon_materials.add(LinearRgba::BLUE),
            ..default()
        },
        RenderLayers::layer(1),
        Name::new("showcase model"),
        DisplayModel,
    ));
}
