use app_core::{plugins::AppSourcesPlugin, ExecLocation, ROOT};
use bevy::{prelude::*, render::{render_resource::{Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages}, view::RenderLayers}, transform::commands};
use bevy_camera_extras::{plugins::DefaultCameraPlugin, FlyCameraSystems};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_picking::{backends::raycast::RaycastBackend, debug::{DebugPickingMode, DebugPickingPlugin}, focus::PickingInteraction, highlight::PickHighlight, picking_core::Pickable, selection::PickSelection, DefaultPickingPlugins, PickableBundle};
use bevy_obj::ObjPlugin;
use bevy_rapier3d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use bevy_serialization_extras::prelude::{
    AssetSpawnRequest, AssetSpawnRequestQueue, PhysicsBundle, PhysicsSerializationPlugin,
    SerializationPlugin,
};
use bevy_serialization_urdf::{
    loaders::urdf_loader::Urdf,
    plugin::{AssetSourcesUrdfPlugin, UrdfSerializationPlugin},
};
use bevy_transform_gizmo::{GizmoTransformable, TransformGizmoPlugin};
use bevy_ui_extras::systems::{visualize_right_sidepanel_for, visualize_window_for};
use robot_editor::{components::DisplayModelCamera, plugins::*, selection_behaviour::plugins::PickingPluginExtras, systems::shape::Cube, ui::{DisplayModel, DisplayModelImage}};
use robot_editor::states::*;




pub fn main() {
    App::new()
        .insert_state(RobotEditorState::Active)
        // app sources
        .add_plugins(AppSourcesPlugin::CRATE)
        .add_plugins(AssetSourcesUrdfPlugin {
            assets_folder_local_path: "../../assets".to_owned(),
        })
        .add_plugins(DefaultPlugins)
        //.add_plugins(WorldInspectorPlugin::default())
        // robot editor
        .add_plugins(RobotEditorPlugin)
        // // serialization plugins
        .add_plugins(SerializationPlugin)
        .add_plugins(PhysicsSerializationPlugin)
        .add_plugins(UrdfSerializationPlugin)
        // // physics
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        // world setup
        .add_systems(Update, visualize_window_for::<DisplayModel>)
        .add_systems(Startup, setup_editor_area)
        .add_systems(PreStartup, second_camera_test)
        //.add_systems(Update, display_model_image_to_file)
        .run();
}

pub fn second_camera_test(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {

    let size = Extent3d {
        width: 512,
        height: 512,
        ..default()
    };


    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    // fill image.data with zeroes
    image.resize(size);


    let image_handle = images.add(image);
    commands.insert_resource(DisplayModelImage(image_handle.clone()));

    //camera
    commands.spawn(
        (
            Camera3dBundle {
                camera: Camera {
                    order: 1,
                    target: image_handle.clone().into(),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 2.5, 4.7).with_rotation(Quat::from_rotation_x(-0.5)),
                ..default()
            },
            RenderLayers::layer(1),
            Name::new("Display Camera"),
            
        )
    );
    // // Cube
    commands.spawn(
        (
            PbrBundle {
                mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
                ..default()
            },
            RenderLayers::layer(1),
            Name::new("showcase_cube"),
            DisplayModelCamera
        )
    );
}