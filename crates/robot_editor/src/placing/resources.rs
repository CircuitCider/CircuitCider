use bevy::{asset::LoadedFolder, prelude::*};

#[derive(Resource, Default, Deref)]
pub struct ModelFolder(pub Handle<LoadedFolder>);