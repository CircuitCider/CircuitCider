pub use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_camera_extras::components::Watched;
use bevy_egui::EguiContext;
use bevy_mod_raycast::{immediate::Raycast, CursorRay};
use bevy_serialization_extras::prelude::{
    link::{JointFlag, StructureFlag},
    rigidbodies::RigidBodyFlag,
};
use crate::{components::*, raycast_utils::systems::{get_first_hit_with_mut, get_first_hit_without, get_first_hit_without_mut}};


/// gets rid of placers if current mode is not placermode
pub fn delete_attach_candidates(
    tool_mode: ResMut<BuildToolMode>,
    placers: Query<Entity, With<AttachCandidate>>,
    mut commands: Commands,
) {
    if *tool_mode != BuildToolMode::EditerMode {
        for e in placers.iter() {
            commands.entity(e).despawn()
        }
    }
}

pub fn move_placer_to_cursor(
    mut raycast: Raycast,
    cursor_ray: Res<CursorRay>,
    tool_mode: ResMut<BuildToolMode>,
    mut placers: Query<&mut Transform, With<Placer>>,
    mouse_over_window: Res<MouseOverWindow>,
) {
    // if let Some(mouse_pos) = **cursor_ray {

    // }
    if *tool_mode == BuildToolMode::PlacerMode {
        //let x = cursor_ray_hititer(cursor_ray, &mut raycast, mouse_over_window).unwrap_or_default();
        //println!("attempting to move placer to cursor");
        
        let filtered_hits = get_first_hit_without_mut(
            cursor_ray_hititer(&cursor_ray, &mut raycast, &mouse_over_window),
            &mut placers,
        );
        println!("hit list: {:#?}", filtered_hits);

        // let hits = cursor_ray_hititer(&cursor_ray, &mut raycast, &mouse_over_window);

        // println!("hits: {:#?}", hits);
        if let Some((e, hit)) = get_first_hit_without_mut(
            cursor_ray_hititer(&cursor_ray, &mut raycast, &mouse_over_window),
            &mut placers,
        ) {
            for mut trans in placers.iter_mut() {
                //println!("moving placer to cursor");
                let hit_pos = hit.position();
                //println!("moving placer to cursor {:#?}", hit_pos);
                trans.translation = hit_pos;
            }
        }
    }
}

/// gets rid of placers if current mode is not placermode
pub fn delete_placers(
    tool_mode: ResMut<BuildToolMode>,
    placers: Query<Entity, With<Placer>>,
    mut commands: Commands,
) {
    if *tool_mode != BuildToolMode::PlacerMode {
        for e in placers.iter() {
            commands.entity(e).despawn()
        }
    }
}

use crate::{
    components::Wheel,
    raycast_utils::{
        resources::MouseOverWindow,
        systems::{cursor_ray_hititer},
    },
    resources::BuildToolMode,
    ui::{AttachCandidate, Placer},
};

pub fn set_robot_to_follow(
    joints: Query<Entity, (With<JointFlag>, Without<Watched>)>,
    mut commands: Commands,
) {
    for e in joints.iter() {
        commands.entity(e).insert(Watched);
    }
}

// pub fn make_robots_editable(
//     unmodified_bots: Query<(Entity, &StructureFlag), Without<Pickable>>,
//     mut commands: Commands,
// ) {
//     for (e, ..) in unmodified_bots.iter() {
//         commands.entity(e)
//         .insert(        bevy_transform_gizmo::GizmoTransformable)
//         .insert(PickableBundle::default())

//         ;
//     }
// }

#[derive(Component)]
pub struct WasFrozen;

pub fn control_robot(
    mut rigid_body_flag: Query<&mut RigidBodyFlag, (Without<JointFlag>, With<StructureFlag>)>,
    keys: Res<ButtonInput<KeyCode>>,
    mut primary_window: Query<&mut EguiContext, With<PrimaryWindow>>,
    mut wheels: Query<(&mut JointFlag, &Wheel)>,
) {
    let target_speed = 20.0;

    let leftward_key = KeyCode::ArrowLeft;
    let rightward_key = KeyCode::ArrowRight;
    let forward_key = KeyCode::ArrowUp;
    let backward_key = KeyCode::ArrowDown;

    let freeze_key = KeyCode::KeyP;
    let unfreeze_key = KeyCode::KeyO;

    for mut context in primary_window.iter_mut() {
        egui::Window::new("robot controls")
            //.frame(DEBUG_FRAME_STYLE)
            .show(context.get_mut(), |ui| {
                ui.label(format!("Freeze key: {:#?}", freeze_key));
                ui.label(format!("unfreeze key {:#?}", unfreeze_key));
                ui.label("-------------------------");
                ui.label("");
                ui.label("wheel controls");
                ui.label(format!("Turn left {:#?}", leftward_key));
                ui.label(format!("Turn right {:#?}", rightward_key));
                ui.label(format!("move forward {:#?}", forward_key));
                ui.label(format!("move backward {:#?}", backward_key));
            });
    }
    for (mut joint, wheel) in wheels.iter_mut() {
        for axis in joint.motors.iter_mut() {
            if keys.pressed(forward_key) {
                axis.target_vel = target_speed
            } else if keys.pressed(backward_key) {
                axis.target_vel = -target_speed
            } else {
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
    mut robots: Query<
        (Entity, &mut RigidBodyFlag),
        (With<StructureFlag>, Without<JointFlag>, Without<WasFrozen>),
    >,
    mut commands: Commands,
) {
    for (e, mut body) in robots.iter_mut() {
        *body = RigidBodyFlag::Fixed;
        commands.entity(e).insert(WasFrozen);
    }
}

/// find what is "probably" the left and right wheel, and give them a marker.
pub fn bind_left_and_right_wheel(
    robots: Query<(Entity, &Name), (With<JointFlag>, Without<Wheel>)>,
    mut commands: Commands,
) {
    for (e, name) in robots.iter() {
        let name_str = name.to_string().to_lowercase();

        let split_up = name_str.split("_").collect::<Vec<&str>>();

        if split_up.contains(&Wheel::Left.to_string().to_lowercase().as_str()) {
            commands.entity(e).insert(Wheel::Left);
        }
        if split_up.contains(&Wheel::Right.to_string().to_lowercase().as_str()) {
            commands.entity(e).insert(Wheel::Right);
        }
    }
}
