use bevy_app::prelude::*;
use bevy_rapier3d::{plugin::RapierPhysicsPlugin, render::RapierDebugRenderPlugin};
use bevy_serialization_assemble::urdf::UrdfSerializationPlugin;
use bevy_serialization_extras::prelude::{SerializationAssembleBasePlugin, SerializationBasePlugin, SerializationPhysicsPlugin, SerializationPlugin};
use combat::weapon_attacks::plugins::CombatPlugin;
use picking_core::plugins::PickingCorePlugin;
use shader_core::plugins::ShaderCorePlugin;
use bevy_rapier3d::prelude::NoUserData;





/// Default plugins for this app. Should consolidate all plugin initialization for this project.
/// !!! LOAD AFTER [`DefaultPlugins`] OR THIS WILL CRASH !!!
pub struct AppDefaultPlugins;

impl Plugin for AppDefaultPlugins {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(ShaderCorePlugin)
        .add_plugins(PickingCorePlugin)
        .add_plugins(SerializationCorePlugin)
        .add_plugins(PhysicsCorePlugin)
        .add_plugins(CombatPlugin)
        ;
    }
}

/// Settings for default physics config
pub struct PhysicsCorePlugin;

impl Plugin for PhysicsCorePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        ;
    }
}

/// plugin for serialization setup.
pub struct SerializationCorePlugin;

impl Plugin for SerializationCorePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(SerializationPlugin)
        .add_plugins(SerializationBasePlugin)
        .add_plugins(SerializationAssembleBasePlugin)
        .add_plugins(SerializationPhysicsPlugin)
        .add_plugins(UrdfSerializationPlugin)
        ;
    }
}