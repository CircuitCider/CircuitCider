use app_core::ROOT;
use bevy::asset::load_internal_asset;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
//use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_raycast::DefaultRaycastingPlugin;
use bevy_serialization_extras::prelude::AssetSpawnRequest;
use bevy_serialization_extras::prelude::AssetSpawnRequestQueue;
use bevy_serialization_extras::prelude::PhysicsBundle;
use bevy_serialization_urdf::loaders::urdf_loader::Urdf;

use crate::raycast_utils::resources::MouseOverWindow;
use crate::resources::BuildToolMode;
use crate::selection_behaviour::plugins::SelectionBehaviourPlugin;
use crate::shaders::neon_glow::NeonGlowMaterial;
use crate::shaders::*;
use crate::states::*;
use crate::systems::*;
use crate::transform_gizmo::plugins::TransformWidgetPlugin;
use crate::ui::ModelFolder;
use crate::ui::*;

pub struct CachePrefabsPlugin;

impl Plugin for CachePrefabsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BuildToolMode::PlacerMode)
            .insert_resource(ModelFolder::default())
            .add_systems(Startup, cache_initial_folders)
            .add_systems(Update, placer_mode_ui)
            .add_systems(Update, select_build_tool);
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

pub struct RobotEditorPlugin;

impl Plugin for RobotEditorPlugin {
    fn build(&self, app: &mut App) {
        // load shaders
        load_internal_asset!(
            app,
            neon_glow::NEON_GLOW_SHADER_HANDLE,
            "shaders/neon_glow.wgsl",
            Shader::from_wgsl
        );
        app.add_plugins(MaterialPlugin::<NeonGlowMaterial>::default());

        app
        
        // asset_loader
        .init_state::<RobotEditorState>()


        // asset folders
        .add_plugins(CachePrefabsPlugin)


        // selection behaviour(what things do when clicked on)
        .add_plugins(SelectionBehaviourPlugin)
        
        .add_plugins(EditorToolingPlugin)

        .init_resource::<MouseOverWindow>()
        .add_systems(PreUpdate, check_if_mouse_over_ui)

        .add_plugins(TransformWidgetPlugin)
        //FIXME: commented out until bevy_inspector_egui is un-broken
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

        //.add_systems(Update, make_robots_editable)
        
        ;
    }
}

pub fn setup_editor_area(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut urdf_load_requests: ResMut<AssetSpawnRequestQueue<Urdf>>,
    cameras: Query<&Camera>,
) {
    println!("setting up editor...");
    
    // don't spawn a camera if there already is one.
    if cameras.iter().len() <= 0 {
        commands.spawn((Camera3dBundle {
            transform: Transform::from_xyz(2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },));
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
