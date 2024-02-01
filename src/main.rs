pub mod ui;

use bevy::{prelude::*, window::PrimaryWindow, input::common_conditions::input_toggle_active};
use bevy_camera_extras::plugins::DefaultCameraPlugin;
use bevy_egui::{EguiPlugin, EguiContext};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::{plugin::{NoUserData, RapierPhysicsPlugin}, render::RapierDebugRenderPlugin};
use bevy_serialization_extras::prelude::{link::{JointFlag, StructureFlag}, rigidbodies::RigidBodyFlag, AssetSpawnRequest, AssetSpawnRequestQueue, PhysicsBundle, PhysicsSerializationPlugin, SerializationPlugin};
use bevy_serialization_urdf::{loaders::urdf_loader::Urdf, plugin::UrdfSerializationPlugin, ui::DEBUG_FRAME_STYLE};
use ui::ui::*;
use bevy_camera_extras::prelude::*;
use bevy_component_extras::components::*;
use bevy_ui_extras::systems::*;
use strum_macros::Display;
//use bevy_flycam::{FlyCam, PlayerPlugin, MovementSettings, NoCameraPlayerPlugin, KeyBindings};


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        // serialization plugins
        .add_plugins(SerializationPlugin)
        .add_plugins(PhysicsSerializationPlugin)
        .add_plugins(UrdfSerializationPlugin)
        
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        
        .add_state::<AppState>()
        //.add_systems(Update, ui_example_system)

        .add_systems(Startup, spawn_start_menu)
        .add_systems(Startup, setup_camera)
        .add_systems(Update, start_arena.run_if(in_state(AppState::MainMenuUI)))
        .add_systems(Update, start_editor.run_if(in_state(AppState::MainMenuUI)))
        .add_systems(Last, exit_app_button.run_if(in_state(AppState::MainMenuUI)))

        //Intro into bevy_serialziation demo
        .add_plugins(
            WorldInspectorPlugin::default().run_if(in_state(AppState::Editor)),
        )
        .add_systems(OnEnter(AppState::Editor), despawn_start_menu)
        .add_systems(OnEnter(AppState::Editor), serialization_demo_setup)
        .add_systems(Update, set_robot_to_follow.run_if(in_state(AppState::Editor)))
        .add_plugins(DefaultCameraPlugin)
        //.add_systems(Update, visualize_right_sidepanel_for::<Name>.run_if(in_state(AppState::Editor)))
        .add_systems(Update, freeze_spawned_robots)
        .add_systems(Update, bind_left_and_right_wheel)
        .add_systems(Update, control_robot.run_if(in_state(AppState::Editor)))
        .insert_resource(KeyBindings {toggle_grab_cursor: KeyCode::ControlLeft, ..default()})
        .run()
        ;
}

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


pub fn serialization_demo_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut urdf_load_requests: ResMut<AssetSpawnRequestQueue<Urdf>>,
) {
    // circular base
    commands.spawn(
        (
            PbrBundle {
                mesh: meshes.add(shape::Plane::from_size(50.0).into()),
                material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
                transform: Transform::from_xyz(0.0, -1.0, 0.0),
                ..default()
            },
            PhysicsBundle::default()
            )
        );
    // urdf_load_requests.requests.push_front(
    //     AssetSpawnRequest {
    //          source: "urdfs/example_bot.xml".to_owned().into(), 
    //          position: Transform::from_xyz(0.0, 1.0, 0.0), 
    //          ..Default::default()
    //     }
    // )
    //;

    urdf_load_requests.requests.push_front(
        AssetSpawnRequest {
                source: "model_pkg/urdf/diff_bot.xml".to_owned().into(), 
                position: Transform::from_xyz(0.0, 15.0, 0.0), 
                ..Default::default()
        }
    )
    ;
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}

#[derive(Component)]
pub struct WasFrozen;

pub fn freeze_spawned_robots(
    mut robots: Query<(Entity, &mut RigidBodyFlag), (With<StructureFlag>, Without<JointFlag>, Without<WasFrozen>)>,
    mut commands: Commands,
) {
    for (e, mut body) in robots.iter_mut() {
        *body = RigidBodyFlag::Fixed;
        commands.entity(e).insert(WasFrozen);
    }
}


#[derive(Component, Reflect, Display)]
pub enum Wheel {
    Left,
    Right,
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

/// set up a simple 3D scene
fn setup_camera(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

// fn ui_example_system(
//     mut primary_window: Query<&mut EguiContext, With<PrimaryWindow>>,

// ) {
//     for mut context in primary_window.iter_mut() {
//         egui::Window::new("Hello").show(context.get_mut(), |ui| {
//             ui.label("click me");
//             if ui.button("world").clicked() {
//                 println!("hello!")
//             }
//         });
//     }
// }