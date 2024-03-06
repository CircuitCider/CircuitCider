
use bevy::asset::load_internal_asset;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
//use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_raycast::DefaultRaycastingPlugin;


use crate::shaders::neon_glow::NeonGlowMaterial;
use crate::shaders::*;
use crate::states::*;
use crate::systems::*;
use crate::transform_gizmo::plugins::TransformWidgetPlugin;
use crate::ui::check_if_mouse_over_ui;
use crate::ui::MouseOverWindow;

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
        app
        .add_plugins(MaterialPlugin::<NeonGlowMaterial>::default())
        ;
        
        app
        
        // asset_loader
        .init_state::<RobotEditorState>()


        // selection behaviour
        //.add_plugins(DefaultPickingPlugins)

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
        //.add_systems(Update, make_robots_editable)
        
        ;
    }
}
