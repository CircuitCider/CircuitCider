use bevy::prelude::*;
use super::systems::*;

pub struct SelectionBehaviourPlugin;

impl Plugin for SelectionBehaviourPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, check_grabs)
        .add_systems(Update, grab_clicked)
        ;
    }
}