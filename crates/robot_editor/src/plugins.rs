use app_core::ROOT;
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy_camera_extras::CameraControllerFree;
use bevy_camera_extras::CameraMode;
use bevy_camera_extras::CameraRestrained;
use bevy_camera_extras::ObservedBy;
//use bevy_camera_extras::components::FlyCam;
//use bevy_camera_extras::components::Watched;
//use bevy_camera_extras::plugins::DefaultCameraPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_picking::debug::DebugPickingMode;
use bevy_mod_picking::focus::PickingInteraction;
//use bevy_mod_raycast::DefaultRaycastingPlugin;
use bevy_serialization_extras::prelude::link::JointFlag;
use bevy_serialization_extras::prelude::AssetSpawnRequest;
use bevy_serialization_extras::prelude::AssetSpawnRequestQueue;
use bevy_serialization_extras::prelude::PhysicsBundle;
use bevy_serialization_urdf::loaders::urdf_loader::Urdf;
use bevy_ui_extras::visualize_resource;
use transform_gizmo_bevy::enum_set;
use transform_gizmo_bevy::GizmoCamera;
use transform_gizmo_bevy::GizmoMode;
use transform_gizmo_bevy::GizmoOptions;
use transform_gizmo_bevy::GizmoOrientation;
use transform_gizmo_bevy::TransformGizmoPlugin;

use crate::attaching::plugins::AttachingToolingPlugin;
use crate::camera_controls::plugins::RobotEditorCameraPlugin;
use crate::model_display::plugins::ModelDisplayerPlugin;
use crate::picking::plugins::PickingPlugin;
use crate::placing::plugins::CachePrefabsPlugin;
use crate::placing::plugins::PlacingToolingPlugin;
use crate::raycast_utils::plugins::CursorRayHitsPlugin;
use crate::raycast_utils::resources::MouseOverWindow;
use crate::resources::RobotControls;
use crate::shaders::*;
use crate::states::*;
use crate::systems::*;
use crate::ui::*;

use self::plugins::CustomShadersPlugin;

/// ui for robot editor
pub struct RobotEditorUiPlugin;

impl Plugin for RobotEditorUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, save_load_model_ui.run_if(in_state(RobotEditorState::Active)));
    }
}

pub struct RobotEditorPlugin;

impl Plugin for RobotEditorPlugin {
    fn build(&self, app: &mut App) {
        app
        // load shaders
        .add_plugins(CustomShadersPlugin)

        // asset_loader
        .init_state::<RobotEditorState>()


        // asset folders
        .add_plugins(CachePrefabsPlugin)

        // Picking
        .add_plugins(RobotEditorCameraPlugin)
        .add_plugins(CursorRayHitsPlugin)
        .register_type::<PickingInteraction>()
        .add_plugins(
            (
            TransformGizmoPlugin,
            PickingPlugin,

            )
        )
        .insert_resource(GizmoOptions {
            gizmo_modes: enum_set!(GizmoMode::RotateX | GizmoMode::RotateY | GizmoMode::RotateZ |GizmoMode::TranslateX | GizmoMode::TranslateY | GizmoMode::TranslateZ),
            gizmo_orientation: GizmoOrientation::Global,
            ..default()
        })
        .insert_resource(DebugPickingMode::Normal)
        .insert_resource(RobotControls::default())
        .register_type::<RobotControls>()
        // selection behaviour(what things do when clicked on)
        
        // build tools
        .add_plugins(
            (
            PlacingToolingPlugin,
            AttachingToolingPlugin,
            )
        )

        .add_plugins(ModelDisplayerPlugin)

        .init_resource::<MouseOverWindow>()

        // ui
        .add_plugins(RobotEditorUiPlugin)
        .add_systems(PreUpdate, check_if_mouse_over_ui)
        .add_plugins(
            WorldInspectorPlugin::default().run_if(in_state(RobotEditorState::Active)),
        )
        //.add_systems(Update, set_robot_to_follow.run_if(in_state(RobotEditorState::Active)))
        .add_systems(Update, control_robot.run_if(in_state(RobotEditorState::Active)))
        .add_systems(Update, freeze_spawned_robots)
        .add_systems(Update, bind_left_and_right_wheel)
        //FIXME: takes 5+ seconds to load like this for whatever reason. Load differently for main and robot_editor to save time.
        //.add_systems(OnEnter(RobotEditorState::Active), setup_editor_area)

        //.add_systems(Update, make_robots_editable)
        
        ;
    }
}

pub fn setup_editor_area(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut urdf_load_requests: ResMut<AssetSpawnRequestQueue<Urdf>>,
    cameras: Query<(Entity, &Camera), With<CameraMode>>,
) {
    println!("setting up editor...");
    match cameras.get_single() {
        Ok(_) => {},
        Err(_) => {
            warn!("multiple cameras with controlers not supported. Despawning extra cameras and spawning new replacement camera");
            for (e, ..) in cameras.iter() {
                commands.entity(e).despawn_recursive();
            }

            commands.spawn((
                Camera3dBundle {
                    transform: Transform::from_xyz(2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
                    ..Default::default()
                },
                CameraControllerFree {
                    restrained: CameraRestrained(true), // attach_to: None,
                                                        // camera_mode: bevy_camera_extras::CameraMode::ThirdPerson(CameraDistanceOffset::default())
                },
                GizmoCamera,
                Name::new("Gizmo Camera"),
                //bevy_transform_gizmo::GizmoPickSource::default(),
                RenderLayers::layer(0),
            ));
        },
    }

    // robot
    urdf_load_requests.requests.push_front(AssetSpawnRequest {
        source: format!("{:#}://model_pkg/urdf/diff_bot.xml", ROOT)
            .to_owned()
            .into(),
        position: Transform::from_xyz(0.0, 15.0, 0.0),
        ..Default::default()
    });

    // plane
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::new(
                Vec3::new(0.0, 1.0, 0.0),
                Vec2::new(50.0, 50.0),
            )),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3)),
            transform: Transform::from_xyz(0.0, -1.0, 0.0),
            ..default()
        },
        PhysicsBundle::default(),
        Name::new("Editor baseplate"),
    ));

    // light
    commands.spawn((PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    },));
}

pub fn set_robot_to_follow(
    cameras: Query<Entity, With<CameraRestrained>>,
    joints: Query<Entity, (With<JointFlag>, Without<ObservedBy>)>,
    mut commands: Commands,
) {
    let Ok(camera) = cameras.get_single() else {
        warn!("multiple cameras found. Skipping");
        return;
    };
    for e in joints.iter() {
        commands.entity(e).insert(ObservedBy(camera));
    }
}
