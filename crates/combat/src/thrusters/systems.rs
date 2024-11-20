use bevy::prelude::*;
use crate::components::{Pistol, Velocity};

const SPEED: f32 = 10.0;
const ROTATION_SPEED: f32 = 2.5;

pub fn movement_controls(
    mut query: Query<(&mut Transform, &mut Velocity), With<Pistol>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let Ok((mut transform, mut velocity)) = query.get_single_mut() else {
        return;
    };
    let mut rotation = 0.0;
    let mut movement = 0.0;

    if keyboard_input.pressed(KeyCode::KeyS) {
        movement = -SPEED;
    } else if keyboard_input.pressed(KeyCode::KeyW) {
        movement = SPEED;
    } // forward and backwards movement controls

    if keyboard_input.pressed(KeyCode::KeyA) {
        rotation = ROTATION_SPEED * time.delta_seconds();
    } else if keyboard_input.pressed(KeyCode::KeyD) {
        rotation = -ROTATION_SPEED * time.delta_seconds();
    } // rotation left and right

    transform.rotate_y(rotation);
    velocity.value = -transform.forward() * movement;
}