use app_core::ROOT;
use bevy::asset::load_internal_asset;
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy_camera_extras::components::Watched;
use bevy_camera_extras::plugins::DefaultCameraPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_picking::debug::DebugPickingMode;
use bevy_mod_picking::debug::DebugPickingPlugin;
use bevy_mod_picking::focus::PickingInteraction;
use bevy_mod_picking::highlight::PickHighlight;
use bevy_mod_picking::picking_core::Pickable;
use bevy_mod_picking::selection::PickSelection;
use bevy_mod_picking::DefaultPickingPlugins;
use bevy_mod_picking::PickableBundle;
//use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_camera_extras::components::FlyCam;
use bevy_mod_raycast::DefaultRaycastingPlugin;
use bevy_serialization_extras::prelude::link::JointFlag;
use bevy_serialization_extras::prelude::AssetSpawnRequest;
use bevy_serialization_extras::prelude::AssetSpawnRequestQueue;
use bevy_serialization_extras::prelude::PhysicsBundle;
use bevy_serialization_urdf::loaders::urdf_loader::Urdf;
use bevy_transform_gizmo::GizmoTransformable;
use bevy_transform_gizmo::TransformGizmoPlugin;

use crate::model_display::plugins::ModelDisplayerPlugin;
use crate::raycast_utils::resources::MouseOverWindow;
use crate::resources::BuildToolMode;
use crate::selection_behaviour::plugins::PickingPluginExtras;
use crate::shaders::neon_glow::NeonGlowMaterial;
use crate::shaders::*;
use crate::states::*;
use crate::systems::*;
use crate::ui::ModelFolder;
use crate::ui::*;

use self::plugins::CustomShadersPlugin;

pub struct CachePrefabsPlugin;

impl Plugin for CachePrefabsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BuildToolMode::PlacerMode)
            //.init_resource::<DisplayModelImage>()
            .insert_resource(ModelFolder::default())
            .add_systems(Startup, cache_initial_folders)
            .add_systems(Update, placer_spawner_ui)
            .add_systems(Update, select_build_tool)
            .add_systems(Update, placer_editor_ui)
            ;
    }
}

/// stuff required to run individual tools of robot editor
pub struct EditorToolingPlugin;

impl Plugin for EditorToolingPlugin {
    fn build(&self, app: &mut App) {
        // placers
        app.add_systems(Update, move_placer_to_cursor)
            .add_systems(Update, attach_placer)
            .add_systems(Update, delete_placers)
            .add_systems(Update, delete_attach_candidates);
    }
}

/// ui for robot editor
pub struct RobotEditorUiPlugin;

impl Plugin for RobotEditorUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, save_load_model_ui);
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
        .add_plugins(DefaultCameraPlugin)
        .register_type::<PickingInteraction>()
        // .add_plugins(
        //     (
        //         DefaultPickingPlugins.build().disable::<DebugPickingPlugin>(),
        //         TransformGizmoPlugin::new(
        //             Quat::from_rotation_y(-0.2), // Align the gizmo to a different coordinate system.
        //                                          // Use TransformGizmoPlugin::default() to align to the
        //                                          // scene's coordinate system.
        //         ),
        //         //PickingPluginExtras
        //     )
        // )
        .insert_resource(DebugPickingMode::Normal)
        // selection behaviour(what things do when clicked on)
        
        .add_plugins(EditorToolingPlugin)

        .add_plugins(ModelDisplayerPlugin)

        .init_resource::<MouseOverWindow>()

        // ui
        .add_plugins(RobotEditorUiPlugin)
        .add_systems(PreUpdate, check_if_mouse_over_ui)
        .add_plugins(
            WorldInspectorPlugin::default().run_if(in_state(RobotEditorState::Active)),
        )
        .add_plugins(
            DefaultRaycastingPlugin,
        )
        //.add_systems(Update, set_robot_to_follow.run_if(in_state(RobotEditorState::Active)))
        .add_systems(Update, control_robot.run_if(in_state(RobotEditorState::Active)))
        .add_systems(Update, freeze_spawned_robots)
        .add_systems(Update, bind_left_and_right_wheel)

        //FIXME: takes 5+ seconds to load like this for whatever reason. Load differently for main and robot_editor to save time.
        //.add_systems(OnEnter(RobotEditorState::Active), setup_editor_area)
        .add_systems(Update, make_models_pickable)

        //.add_systems(Update, make_robots_editable)
        
        ;
    }
}

pub fn setup_editor_area(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut urdf_load_requests: ResMut<AssetSpawnRequestQueue<Urdf>>,
    cameras: Query<(Entity, &Camera)>,
) {
    println!("setting up editor...");
    // for (e, ..) in cameras.iter() {
    //     commands.entity(e).despawn_recursive();
    // }
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        FlyCam,
        bevy_transform_gizmo::GizmoPickSource::default(),
        RenderLayers::layer(0),
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
            mesh: meshes.add(
                Plane3d::new(Vec3::new(0.0, 1.0, 0.0))
                    .mesh()
                    .size(50.0, 50.0),
            ),
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

pub fn make_models_pickable(
    mut commands: Commands,
    models_query: Query<Entity, (With<Handle<Mesh>>, Without<Pickable>)>,
) {
    for e in models_query.iter() {
        commands.entity(e).insert((
            PickableBundle {
                pickable: Pickable::default(),
                interaction: PickingInteraction::default(),
                selection: PickSelection::default(),
                highlight: PickHighlight::default(),
            },
            GizmoTransformable,
        ));
    }
}

pub fn set_robot_to_follow(
    joints: Query<Entity, (With<JointFlag>, Without<Watched>)>,
    mut commands: Commands,
) {
    for e in joints.iter() {
        commands.entity(e).insert(Watched);
    }
}
