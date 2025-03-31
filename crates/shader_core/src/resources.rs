use bevy_asset::{LoadedFolder, prelude::*};
use bevy_derive::{Deref, DerefMut};
use bevy_ecs::prelude::*;
use bevy_utils::HashMap;
use derive_more::From;

#[derive(Resource, Default, Deref, From)]
pub struct ShadersFolder(pub Handle<LoadedFolder>);

/// cache of loaded wgsl shaders
#[derive(Resource, Default, Deref, DerefMut)]
pub struct WgslCache(HashMap<String, String>);
