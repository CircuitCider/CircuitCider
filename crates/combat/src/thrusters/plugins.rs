use bevy::prelude::*;
use super::systems::*;
pub struct ThrustersPlugin;

impl Plugin for ThrustersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, movement_controls);
    }
}