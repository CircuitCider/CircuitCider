use std::f32::consts::PI;

use app_core::ROOT;
use assembling::plugins::AssemblingPlugin;
use attaching::plugins::AttachingToolingPlugin;
use bevy::core_pipeline::Skybox;
use bevy::pbr::wireframe::WireframePlugin;
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy_asset_loader::asset_collection::AssetCollectionApp;
use bevy_camera_extras::CameraController;
use bevy_camera_extras::CameraMode;
use bevy_camera_extras::CameraRestrained;
use bevy_camera_extras::ObservedBy;
use bevy_mod_picking::backends::raycast::RaycastPickable;
use bevy_mod_picking::debug::DebugPickingMode;
use bevy_mod_picking::focus::PickingInteraction;
use bevy_mod_raycast::prelude::RaycastSource;
use bevy_rapier3d::plugin::NoUserData;
use bevy_rapier3d::plugin::RapierPhysicsPlugin;
//use bevy_mod_raycast::DefaultRaycastingPlugin;
use bevy_serialization_extras::prelude::link::JointFlag;
use bevy_serialization_extras::prelude::mesh::GeometryFile;
use bevy_serialization_extras::prelude::mesh::GeometryFlag;
use bevy_serialization_extras::prelude::AssetSpawnRequest;
use bevy_serialization_extras::prelude::AssetSpawnRequestQueue;
use bevy_serialization_extras::prelude::DeserializeAssetFrom;
use bevy_serialization_extras::prelude::PhysicsBundle;
use bevy_serialization_extras::prelude::SerializationPhysicsPlugin;
use bevy_serialization_extras::prelude::SerializationPlugin;
use bevy_serialization_urdf::loaders::urdf_loader::Urdf;
use bevy_serialization_urdf::plugin::UrdfSerializationPlugin;
use bevy_toon_shader::ToonShaderMainCamera;
use bevy_toon_shader::ToonShaderPlugin;
use bevy_toon_shader::ToonShaderSun;
use camera_controls::plugins::RobotEditorCameraPlugin;
use components::Wheel;
use model_display::plugins::ModelDisplayerPlugin;
use picking::plugins::PickingPlugin;
use placing::plugins::PlacingToolingPlugin;
use raycast_utils::plugins::CursorRayCam;
use raycast_utils::plugins::CursorRayHitsPlugin;
use raycast_utils::resources::MouseOverWindow;
use resources::BuildMenuTarget;
use resources::BuildToolMode;
use resources::ImageHandles;
use resources::RobotControls;
use resources::WeaponsFolder;
use resources::WheelsFolder;
use shader_core::shaders::plugins::CustomShadersPlugin;
use states::RobotEditorState;
use systems::configure_skybox_texture;
use transform_gizmo_bevy::enum_set;
use transform_gizmo_bevy::GizmoCamera;
use transform_gizmo_bevy::GizmoMode;
use transform_gizmo_bevy::GizmoOptions;
use transform_gizmo_bevy::GizmoOrientation;
use transform_gizmo_bevy::TransformGizmoPlugin;
use ui::*;
use systems::*;

use super::*;

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
        .add_plugins(WireframePlugin)
        .register_type::<Wheel>()
        // load shaders
        .add_plugins(CustomShadersPlugin)
        .add_plugins(ToonShaderPlugin)
        .init_collection::<ImageHandles>()

        .add_systems(Update, configure_skybox_texture)
        .init_state::<RobotEditorState>()


        // asset folders
        .add_plugins(CachePrefabsPlugin)

        // PickingRobotEditorPlugin
        .add_plugins(RobotEditorCameraPlugin)
        .add_plugins(CursorRayHitsPlugin {debug_mode: false})
        .register_type::<PickingInteraction>()
        .add_plugins(
            (
            TransformGizmoPlugin,
            PickingPlugin,
            )
        )

        // Serialization
        .add_plugins(SerializationPlugin)
        .add_plugins(DeserializeAssetFrom::<GeometryFlag, Mesh>::default())
        .add_plugins(DeserializeAssetFrom::<GeometryFile, Mesh>::default())

        .add_plugins(SerializationPhysicsPlugin)
        .add_plugins(UrdfSerializationPlugin)
        
        .insert_resource(GizmoOptions {
            gizmo_modes: enum_set!(GizmoMode::RotateX | GizmoMode::RotateY | GizmoMode::RotateZ |GizmoMode::TranslateX | GizmoMode::TranslateY | GizmoMode::TranslateZ),
            gizmo_orientation: GizmoOrientation::Global,
            ..default()
        })
        .insert_resource(DebugPickingMode::Normal)
        .insert_resource(RobotControls::default())
        .register_type::<RobotControls>()        
        .insert_resource(BuildMenuTarget::default())
        // build tools
        .add_plugins(
            (
            PlacingToolingPlugin,
            AttachingToolingPlugin,
            AssemblingPlugin,
            )
        )
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())

        .add_plugins(ModelDisplayerPlugin)

        .init_resource::<MouseOverWindow>()

        // ui
        .add_plugins(RobotEditorUiPlugin)
        .add_systems(PreUpdate, check_if_mouse_over_ui)

        .add_systems(Update, control_robot.run_if(in_state(RobotEditorState::Active)))
        .add_systems(Update, freeze_spawned_robots)
        .add_systems(Update, bind_left_and_right_wheel)
        .add_systems(Update, set_robot_to_toon_shader)
        .add_systems(Startup, spawn_toon_shader_cam)
        //.add_systems(Update, build_menu_ui)
        //FIXME: takes 5+ seconds to load like this for whatever reason. Load differently for main and robot_editor to save time.
        .add_systems(OnEnter(RobotEditorState::Active), setup_editor_area)
        .add_systems(Update, build_menu_ui.run_if(in_state(RobotEditorState::Active)));


        
        ;
    }
}

pub struct CachePrefabsPlugin;

impl Plugin for CachePrefabsPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_state::<BuildToolMode>()
            //.init_resource::<DisplayModelImage>()
            .insert_resource(HullsFolder::default())
            .insert_resource(WeaponsFolder::default())
            .insert_resource(WheelsFolder::default())
            .add_systems(Startup, cache_initial_folders)
            //.add_systems(Update, select_build_tool)
            
            ;
    }
}

fn setup_editor_area(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut urdf_load_requests: ResMut<AssetSpawnRequestQueue<Urdf>>,
    images: Res<ImageHandles>,
    mut cameras: Query<(Entity, &mut CameraMode, Option<&mut Name>), With<Camera>>
) {
    println!("setting up editor...");
    
    let cam = match cameras.get_single_mut() {
        Ok(cam) => Some(cam),
        Err(err) => {
            match err {
                bevy::ecs::query::QuerySingleError::NoEntities(err) => {
                    warn!("No camera found. Creating new one: Actual error: {:#}", err);
                    None
                    // (commands.spawn_empty().id(), CameraMode::Free, None)
                },
                bevy::ecs::query::QuerySingleError::MultipleEntities(err) => {
                    warn!("multiple cameras found. Aborting setup {:#}", err);
                    return;
                },
            }
        }
    };
    let cam = match cam {
        Some((e, mut mode, name)) => {
            *mode = CameraMode::Free;
            e
        },
        None => {
            commands.spawn_empty()
            .insert(
                (
                CameraMode::Free,
                Name::new("editor cam"),
            ))
            .id()
        }
    };

    commands.entity(cam).insert((
        Camera3dBundle {
            transform: Transform::from_xyz(2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            camera: Camera {
                order: 2, 
                ..default()
            },
            ..Default::default()
            
        },
        CameraRestrained(false),
        ToonShaderMainCamera,
        GizmoCamera,
        RaycastPickable,
        CursorRayCam, // Set this camera as a raycaster using the mouse cursor

        Name::new("editor cam"),
        RenderLayers::layer(0),
        Skybox {
            image: images.skybox.clone(),
            brightness: 1000.0
        }
    ));

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
            material: materials.add(Color::srgb(0.3, 0.5, 0.3)),
            transform: Transform::from_xyz(0.0, -1.0, 0.0),
            ..default()
        },
        PhysicsBundle::default(),
        Name::new("Editor baseplate"),
    ));
    // Sun
    commands
    .spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                shadows_enabled: true,
                illuminance: 10_000.,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(2.0, 2.0, 2.0),
                rotation: Quat::from_euler(EulerRot::XYZ, -PI / 4., PI / 6., 0.),
                ..default()
            },
            ..default()
        },
        ToonShaderSun,
        Name::new("Sun")
    ));
    // light
    commands
    .spawn((
        PointLightBundle {
            point_light: PointLight {
                intensity: 1500.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..default()
        },
    )
);
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

