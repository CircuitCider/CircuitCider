use bevy_app::prelude::*;
use bevy_ui_extras::UiExtrasDebug;



/// Default plugins for this app. Should consolidate all plugin initialization for this project.
/// !!! LOAD AFTER [`DefaultPlugins`] OR THIS WILL CRASH !!!
pub struct AppDefaultPlugins;

impl Plugin for AppDefaultPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins(UiExtrasDebug::default());
        #[cfg(feature = "shaders")]
        {
            use shader_core::plugins::ShaderCorePlugin;
            app.add_plugins(ShaderCorePlugin);
        }
        #[cfg(feature = "picking")] 
        {
            use picking_core::plugins::PickingCorePlugin;
            app.add_plugins(PickingCorePlugin);
        }
        #[cfg(feature = "physics")]
        {
            use physics::PhysicsCorePlugin;
            app.add_plugins(physics::PhysicsCorePlugin);
        }
        #[cfg(feature = "serialization")] {
            use serialization::SerializationCorePlugin;
            app.add_plugins(SerializationCorePlugin);
        }
        #[cfg(feature = "combat")] {
            use combat::weapon_attacks::plugins::CombatPlugin;
            app.add_plugins(CombatPlugin);
        }
        #[cfg(feature = "robot_editor")] {
            use robot_editor::plugins::RobotEditorPlugin;
            app.add_plugins(RobotEditorPlugin);
        }
    }
}
#[cfg(feature = "physics")]
pub mod physics {
    use bevy_app::{App, Plugin};
    use bevy_rapier3d::prelude::NoUserData;
    use bevy_rapier3d::{plugin::RapierPhysicsPlugin, render::RapierDebugRenderPlugin};

    /// Settings for default physics config
    pub struct PhysicsCorePlugin;

    impl Plugin for PhysicsCorePlugin {
        fn build(&self, app: &mut App) {
            app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
                .add_plugins(RapierDebugRenderPlugin::default());
        }
    }
}

#[cfg(feature = "serialization")]
pub mod serialization {
    use bevy_app::{App, Plugin};
    use bevy_serialization_assemble::urdf::UrdfSerializationPlugin;
    use bevy_serialization_extras::prelude::{
        SerializationAssembleBasePlugin, SerializationBasePlugin, SerializationPhysicsPlugin,
        SerializationPlugin,
    };
    /// plugin for serialization setup.
    pub struct SerializationCorePlugin;

    impl Plugin for SerializationCorePlugin {
        fn build(&self, app: &mut App) {
            app.add_plugins(SerializationPlugin)
                .add_plugins(SerializationBasePlugin)
                .add_plugins(SerializationAssembleBasePlugin)
                .add_plugins(SerializationPhysicsPlugin)
                .add_plugins(UrdfSerializationPlugin);
        }
    }
}


