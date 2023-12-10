pub mod ui;
pub mod serialization;

use bevy::{prelude::*, window::PrimaryWindow, input::common_conditions::input_toggle_active};
use bevy_camera_extras::plugins::DefaultCameraPlugin;
use bevy_egui::{EguiPlugin, EguiContext};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_serialization_extras::{resources::{AssetSpawnRequestQueue, AssetSpawnRequest}, loaders::urdf_loader::Urdf, plugins::SerializationPlugin, wrappers::link::JointFlag};
use ui::ui::*;
use bevy_camera_extras::prelude::*;
use bevy_component_extras::components::*;
use bevy_ui_extras::systems::*;
//use bevy_flycam::{FlyCam, PlayerPlugin, MovementSettings, NoCameraPlayerPlugin, KeyBindings};


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(SerializationPlugin)
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
        .add_systems(Update, visualize_right_sidepanel_for::<Name>.run_if(in_state(AppState::Editor)))
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

// pub fn attach_fly_cam(
//     camera_query: Query<Entity, With<Camera>>,
//     mut commands: Commands,

// ) {
//     for e in camera_query.iter() {
//         commands.entity(e)
//         .insert(FlyCam)
//         ;
//     }
// }

pub fn serialization_demo_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut urdf_load_requests: ResMut<AssetSpawnRequestQueue<Urdf>>,
) {
    // circular base
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Circle::new(4.0).into()),
        material: materials.add(Color::SEA_GREEN.into()),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });
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
                source: "urdfs/tutorial_bot.xml".to_owned().into(), 
                position: Transform::from_xyz(0.0, 1.0, 0.0), 
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