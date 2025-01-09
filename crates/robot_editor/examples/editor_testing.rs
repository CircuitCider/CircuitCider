use app_core::plugins::AppSourcesPlugin;
use bevy::prelude::*;
use bevy_serialization_extras::prelude::*;
use bevy_ui_extras::*;
use combat::weapon_attacks::plugins::CombatPlugin;
use egui::Align2;
use robot_editor::{
    plugins::*,
    states::RobotEditorState,
};

pub fn main() {
    App::new()
        .add_plugins(AppSourcesPlugin::CRATE)
        .add_plugins(AssetSourcesUrdfPlugin {
            assets_folder_local_path: "../../assets".to_owned(),
        })
        .add_plugins(CombatPlugin)
        .add_plugins(DefaultPlugins)
        //.add_plugins(DefaultPlugins.set(bevy_mod_raycast::low_latency_window_plugin()))
        .insert_state(RobotEditorState::Active)
        // robot editor
        .add_plugins(RobotEditorPlugin)
        .add_plugins(UiExtrasDebug {
            ui_style: UiStyle::BLACK_GLASS,
            alignment: UiAlignment(Align2::RIGHT_TOP),
            ..default()
        })
        .run();
}
