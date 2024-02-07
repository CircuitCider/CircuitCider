pub use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_component_extras::components::Watched;
use bevy_egui::EguiContext;
use bevy_mod_picking::{picking_core::Pickable, PickableBundle};
use bevy_serialization_extras::prelude::{link::{JointFlag, StructureFlag}, rigidbodies::RigidBodyFlag, *};
use bevy_serialization_urdf::loaders::urdf_loader::Urdf;
use bevy_ui_extras::stylesheets::DEBUG_FRAME_STYLE;
use strum_macros::Display;

use crate::components::Wheel;

pub fn set_robot_to_follow(
    joints: Query<Entity, (With<JointFlag>, Without<Watched>)>,
    mut commands: Commands,
) {
    for e in joints.iter() {
        commands.entity(e)
        .insert(Watched)
        ;
    }
}

pub fn make_robots_editable(
    unmodified_bots: Query<(Entity, &StructureFlag), Without<Pickable>>,
    mut commands: Commands,
) {
    for (e, ..) in unmodified_bots.iter() {
        commands.entity(e)
        .insert(        bevy_transform_gizmo::GizmoTransformable)
        .insert(PickableBundle::default())

        ;
    }
}

#[derive(Component)]
pub struct WasFrozen;

pub fn control_robot(
    mut rigid_body_flag: Query<(&mut RigidBodyFlag), (Without<JointFlag>, With<StructureFlag>)>,
    keys: Res<Input<KeyCode>>,
    mut primary_window: Query<&mut EguiContext, With<PrimaryWindow>>,
    mut wheels: Query<(&mut JointFlag, &Wheel)>

) {
    let target_speed = 20.0;

    let leftward_key = KeyCode::Left;
    let rightward_key = KeyCode::Right;
    let forward_key = KeyCode::Up;
    let backward_key = KeyCode::Down;



    let freeze_key = KeyCode::P;
    let unfreeze_key = KeyCode::O;

    for mut context in primary_window.iter_mut() {
        egui::Window::new("robot controls")
        .frame(DEBUG_FRAME_STYLE)
        .show(context.get_mut(),|ui|{   
            ui.label(format!("Freeze key: {:#?}", freeze_key));
            ui.label(format!("unfreeze key {:#?}", unfreeze_key));
            ui.label("-------------------------");
            ui.label("");
            ui.label("wheel controls");
            ui.label(format!("Turn left {:#?}", leftward_key));
            ui.label(format!("Turn right {:#?}", rightward_key));
            ui.label(format!("move forward {:#?}", forward_key));
            ui.label(format!("move backward {:#?}", backward_key));



        })
        ;
    }
    for (mut joint, wheel) in wheels.iter_mut() {
        for axis in joint.motors.iter_mut() {
            if keys.pressed(forward_key) {
                axis.target_vel = target_speed
            } 
            else if keys.pressed(backward_key) {
                axis.target_vel = -target_speed
            }
            else {
                axis.target_vel = 0.0
            }
        }
        match wheel {
            
            Wheel::Left => {
                for axis in joint.motors.iter_mut() {
                    if keys.pressed(leftward_key) {
                        axis.target_vel = -target_speed
                    }
                    if keys.pressed(rightward_key) {
                        axis.target_vel = target_speed
                    }
                }
            }
            Wheel::Right => {
                for axis in joint.motors.iter_mut() {
                    if keys.pressed(leftward_key) {
                        axis.target_vel = target_speed
                    }
                    if keys.pressed(rightward_key) {
                        axis.target_vel = -target_speed
                    }
                }
            }
            
        }

    }

    if keys.pressed(freeze_key) {
        for mut rigidbody in rigid_body_flag.iter_mut() {
            *rigidbody = RigidBodyFlag::Fixed;
        }
    }
    if keys.pressed(unfreeze_key) {
        for mut rigidbody in rigid_body_flag.iter_mut() {
            *rigidbody = RigidBodyFlag::Dynamic;
        }
    }
}

pub fn freeze_spawned_robots(
    mut robots: Query<(Entity, &mut RigidBodyFlag), (With<StructureFlag>, Without<JointFlag>, Without<WasFrozen>)>,
    mut commands: Commands,
) {
    for (e, mut body) in robots.iter_mut() {
        *body = RigidBodyFlag::Fixed;
        commands.entity(e).insert(WasFrozen);
    }
}




// #[derive(Component)]
// pub struct WheelLeft;

// #[derive(Component)]
// pub struct WheelRight;

/// find what is "probably" the left and right wheel, and give them a marker.
pub fn bind_left_and_right_wheel(
    robots: Query<(Entity, &Name), (With<JointFlag>, Without<Wheel>)>,
    mut commands: Commands,
) {
    for (e, name) in robots.iter() {
        let name_str = name.to_string().to_lowercase();
    
        let split_up = name_str.split("_")
        .collect::<Vec<&str>>()
        ;

        if split_up.contains(
            &Wheel::Left
            .to_string()
            .to_lowercase()
            .as_str()) {
            commands.entity(e).insert(Wheel::Left);
        }
        if split_up.contains(
            &Wheel::Right
            .to_string()
            .to_lowercase()
            .as_str()) {
            commands.entity(e).insert(Wheel::Right);
        }
    }
}