use bevy::asset::io::file::FileAssetReader;
use bevy::asset::io::AssetSource;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::systems::*;
use crate::states::*;

pub struct RobotEditorPlugin;

impl Plugin for RobotEditorPlugin {
    fn build(&self, app: &mut App) {
        app
        // asset_loader
        .add_state::<RobotEditorState>()

        .add_plugins(
            WorldInspectorPlugin::default().run_if(in_state(RobotEditorState::Active)),
        )
        .add_systems(Update, set_robot_to_follow.run_if(in_state(RobotEditorState::Active)))
        .add_systems(Update, control_robot.run_if(in_state(RobotEditorState::Active)))
        .add_systems(Update, freeze_spawned_robots)
        .add_systems(Update, bind_left_and_right_wheel)
        
        ;
    }
}