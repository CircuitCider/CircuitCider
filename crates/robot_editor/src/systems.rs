use crate::{
    components::*,
    raycast_utils::systems::{
        get_first_hit_with_mut, get_first_hit_without, get_first_hit_without_mut,
    },
    shaders::neon_glow::NeonGlowMaterial,
    ui::Edited,
};
pub use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_camera_extras::components::Watched;
use bevy_egui::EguiContext;
use bevy_mod_raycast::{immediate::Raycast, CursorRay};
use bevy_rapier3d::plugin::RapierContext;
use bevy_serialization_extras::prelude::{
    link::{JointFlag, StructureFlag},
    rigidbodies::RigidBodyFlag,
};

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
        if let Some((.., hit)) = get_first_hit_without_mut(
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
    raycast_utils::{resources::MouseOverWindow, systems::cursor_ray_hititer},
    resources::BuildToolMode,
    ui::{AttachCandidate, Placer},
};

/// checks for any intersection between the placer and other meshes
pub fn attach_placer(
    //mut raycast: Raycast,
    //cursor_ray: Res<CursorRay>,
    rapier_context: Res<RapierContext>,
    mut neon_materials: ResMut<Assets<NeonGlowMaterial>>,
    placers: Query<(
        Entity,
        &Handle<NeonGlowMaterial>,
        &Handle<Mesh>,
        &Transform,
        &Placer,
    )>,
    mouse: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut tool_mode: ResMut<BuildToolMode>,
    mouse_over_window: Res<MouseOverWindow>,
) {
    if mouse.just_pressed(MouseButton::Left) && **mouse_over_window == false {
        for (e, handle, mesh, trans, ..) in placers.iter() {
            if let Some(mat) = neon_materials.get_mut(handle) {
                if rapier_context
                    .intersection_pairs_with(e)
                    .collect::<Vec<_>>()
                    .len()
                    > 0
                {
                    *mat = Color::RED.into();
                } else {
                    *mat = Color::GREEN.into();
                }
                println!("placing placer..");

                commands.spawn((
                    MaterialMeshBundle {
                        mesh: mesh.clone(),
                        material: handle.clone(),
                        transform: *trans,
                        ..default()
                    },
                    Edited,
                    AttachCandidate,
                ));
                *tool_mode = BuildToolMode::EditerMode;
            }
        }
    }
    if keys.just_pressed(KeyCode::Escape) {
        for (e, ..) in placers.iter() {
            commands.entity(e).despawn_recursive();
        }
    }
}

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
