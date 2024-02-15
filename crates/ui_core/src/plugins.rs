use bevy::{
    asset::io::{file::FileAssetReader, AssetSource},
    prelude::*,
};

use crate::{states::MainMenuState, ui::*};

/// all of the systems for the startmenu
pub struct StartMenuPlugin;

impl Plugin for StartMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<MainMenuState>()
            .add_systems(Startup, spawn_start_menu)
            .add_systems(Update, start_arena.run_if(in_state(MainMenuState::Active)))
            .add_systems(Update, start_editor.run_if(in_state(MainMenuState::Active)))
            .add_systems(
                Last,
                exit_app_button.run_if(in_state(MainMenuState::Active)),
            )
            .add_systems(OnEnter(MainMenuState::Inactive), despawn_start_menu);
    }
}
