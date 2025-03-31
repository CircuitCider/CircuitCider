use bevy_core::Name;
use bevy_core_pipeline::core_3d::Camera3d;
use bevy_ecs::prelude::*;
use bevy_math::Vec3;
use bevy_render::camera::Camera;
use bevy_transform::components::Transform;
use bevy_utils::default;

pub fn spawn_toon_shader_cam(mut commands: Commands) {
    commands.spawn((
        Camera {
            hdr: true,
            order: 1,
            ..default()
        },
        Camera3d::default(),
        Transform::from_xyz(0.0, 8., 12.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
        Name::new("toon camera"),
    ));
}