use crate::{
    resources::RobotControls,
};
pub use bevy::prelude::*;
//use bevy_camera_extras::Watched;
use bevy_rapier3d::plugin::RapierContext;
use bevy_serialization_extras::prelude::{
    link::{JointFlag, StructureFlag},
    rigidbodies::RigidBodyFlag,
};
use bevy_toon_shader::{ToonShaderMainCamera, ToonShaderMaterial, ToonShaderSun};

use crate::{
    components::Wheel,
};

/// find models with given component, and change their material based on if it has any intersections or not.
pub fn intersection_colors_for<T: Component, U: Material>(
    rapier_context: Res<RapierContext>,
    thing_query: Query<(Entity, &Handle<U>), With<T>>,
    mut materials: ResMut<Assets<U>>,
) where
    U: From<LinearRgba>,
{
    for (e, mat_handle) in thing_query.iter() {
        let Some(mat) = materials.get_mut(mat_handle) else {
            return;
        };

        if rapier_context
            .intersection_pairs_with(e)
            .collect::<Vec<_>>()
            .len()
            > 0
        {
            *mat = LinearRgba::RED.into();
        } else {
            *mat = LinearRgba::GREEN.into();
        }
    }
}

#[derive(Component)]
pub struct WasFrozen;

pub fn control_robot(
    mut rigid_body_flag: Query<&mut RigidBodyFlag, (Without<JointFlag>, With<StructureFlag>)>,
    keys: Res<ButtonInput<KeyCode>>,
    controls: Res<RobotControls>,
    //mut primary_window: Query<&mut EguiContext, With<PrimaryWindow>>,
    mut wheels: Query<(&mut JointFlag, &Wheel)>,
) {
    // let target_speed = 20.0;

    // let leftward_key = KeyCode::ArrowLeft;
    // let rightward_key = KeyCode::ArrowRight;
    // let forward_key = KeyCode::ArrowUp;
    // let backward_key = KeyCode::ArrowDown;

    // let freeze_key = KeyCode::KeyP;
    // let unfreeze_key = KeyCode::KeyO;

    // for mut context in primary_window.iter_mut() {
    //     egui::Window::new("robot controls")
    //         //.frame(DEBUG_FRAME_STYLE)
    //         .show(context.get_mut(), |ui| {
    //             ui.label(format!("Freeze key: {:#?}", freeze_key));
    //             ui.label(format!("unfreeze key {:#?}", unfreeze_key));
    //             ui.label("-------------------------");
    //             ui.label("");
    //             ui.label("wheel controls");
    //             ui.label(format!("Turn left {:#?}", leftward_key));
    //             ui.label(format!("Turn right {:#?}", rightward_key));
    //             ui.label(format!("move forward {:#?}", forward_key));
    //             ui.label(format!("move backward {:#?}", backward_key));
    //         });
    // }
    for (mut joint, wheel) in wheels.iter_mut() {
        for axis in joint.motors.iter_mut() {
            if keys.pressed(controls.forward_key) {
                axis.target_vel = controls.target_speed
            } else if keys.pressed(controls.backward_key) {
                axis.target_vel = -controls.target_speed
            } else {
                axis.target_vel = 0.0
            }
        }
        match wheel {
            Wheel::Left => {
                for axis in joint.motors.iter_mut() {
                    if keys.pressed(controls.leftward_key) {
                        axis.target_vel = -controls.target_speed
                    }
                    if keys.pressed(controls.rightward_key) {
                        axis.target_vel = controls.target_speed
                    }
                }
            }
            Wheel::Right => {
                for axis in joint.motors.iter_mut() {
                    if keys.pressed(controls.leftward_key) {
                        axis.target_vel = controls.target_speed
                    }
                    if keys.pressed(controls.rightward_key) {
                        axis.target_vel = -controls.target_speed
                    }
                }
            }
        }
    }

    if keys.pressed(controls.freeze_key) {
        for mut rigidbody in rigid_body_flag.iter_mut() {
            *rigidbody = RigidBodyFlag::Fixed;
        }
    }
    if keys.pressed(controls.unfreeze_key) {
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

pub fn spawn_toon_shader_cam(
    mut commands: Commands,
) {
    commands.spawn(
        (
            Camera3dBundle {
                camera: Camera {
                    hdr: true,
                    order: 1,
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 8., 12.0)
                    .looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
                ..default()
            },
            ToonShaderMainCamera,
            Name::new("toon camera"),
        )
    );  

}

/// change robots to use toon shader 
pub fn set_robot_to_toon_shader(
    mut commands: Commands,
    standard_mats: ResMut<Assets<StandardMaterial>>,
    mut toon_mats: ResMut<Assets<ToonShaderMaterial>>,
    bots: Query<(Entity, Option<&Handle<StandardMaterial>>), (With<StructureFlag>, Without<Handle<ToonShaderMaterial>>)>,
) {
    for (bot, handle) in bots.iter() {
        println!("setting {:#} to toon shader..", bot);
        let mat = match handle {
            Some(mat) => {
                standard_mats.get(mat).unwrap()
            }
            None => &StandardMaterial::default()
        };
        let toon_mat = toon_mats.add(ToonShaderMaterial {
            color: mat.base_color,
            sun_dir: Vec3::default(),
            sun_color: Color::LinearRgba(LinearRgba::WHITE),
            camera_pos: Vec3::default(),
            ambient_color: Color::LinearRgba(LinearRgba::WHITE),
            base_color_texture: None,
        });
        commands.entity(bot).insert(
            toon_mat
        );

        commands.entity(bot).remove::<Handle<StandardMaterial>>();
        

    }
}
