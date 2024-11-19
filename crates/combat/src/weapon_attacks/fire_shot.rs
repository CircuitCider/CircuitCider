use bevy::prelude::*;
use bevy_rapier3d::prelude::{Collider, Sensor};
use crate::components::{Health, Pistol, Bullet, Velocity, CollisionDamage};

pub fn fire_bullet(
    mut commands: Commands,
    query: Query<&mut Transform, With<Pistol>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let Ok(transform) = query.get_single() else {
        return;
    };
    if keyboard_input.pressed(KeyCode::KeyF) {
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Cuboid::new(0.1, 0.1, 0.1)),
                material: materials.add(Color::BLACK),
                transform: Transform::from_translation(
                    transform.translation + -transform.forward() * 1.0,
                ),
                ..default()
            },
            Health::new(1.0),
            CollisionDamage(25.0),
            Velocity::new(-transform.forward() * 25.0),
            Collider::cuboid(0.05, 0.05, 0.05),
            Bullet,
            Sensor,
            Name::new("bullet"),
        ));
    }
}