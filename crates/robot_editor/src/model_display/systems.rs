use bevy::{prelude::*, render::view::RenderLayers};
use shader_core::shaders::neon::NeonMaterial;

use super::{components::*, plugins::DISPLAY_MODEL_TRANSLATION, DisplayModel};

/// enviorment display models are showcased in.
pub fn setup_display_area(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    //camera
    commands.spawn((
        Camera {
            order: 4,
            ..default()
        },
        Camera3d::default(),
        Transform::from_xyz(0.0, 2.5, 4.7).with_rotation(Quat::from_rotation_x(-0.5)),
        RenderLayers::layer(1),
        Name::new("Display Camera"),
    ));

    // plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::new(
            Vec3::new(0.0, 1.0, 0.0),
            Vec2::new(50.0, 50.0),
        ))),
        MeshMaterial3d(materials.add(Color::LinearRgba(LinearRgba::GREEN))),
        Transform::from_xyz(0.0, -47.2, -91.5),
        RenderLayers::layer(1),
        Name::new("Display Floor"),
        DisplayModelStaging,
    ));
}

pub fn setup_display_model(
    display_model: Res<DisplayModel>,
    models: Query<(&Mesh3d, Option<&MeshMaterial3d<StandardMaterial>>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
    children: Query<&Children>
) {
    let Some(source) = display_model.0 else {
        warn!("This should be unreachable?");
        return;
    };
    let source_children = {
        let mut childs = Vec::new();

        if let Ok(children) = children.get(source) {
            for child in children {
                childs.push(child)
            }
        } 
        childs
        // = children.get(root)
    };
    let root = commands.spawn(
        (
            DisplayRoot,
            Name::new("display model"),
            Transform::from_translation(DISPLAY_MODEL_TRANSLATION),
            RenderLayers::layer(1),


        )
    ).id();
    if let Ok((mesh, mat)) = models.get(source) {
        commands.entity(root).insert(mesh.clone());
        if let Some(mat) = mat {
            commands.entity(root).insert(mat.clone());
        } else {
            let handle = materials.add(StandardMaterial::default());
            commands.entity(root).insert(MeshMaterial3d(handle));
        }
    }
    for source_child in source_children {
        if let Ok((mesh, mat)) = models.get(*source_child) {
            let child = commands.spawn(
                mesh.clone()
            ).id();
            if let Some(mat) = mat {
                commands.entity(child).insert(mat.clone());
            } else {
                let handle = materials.add(StandardMaterial::default());
                commands.entity(child).insert(MeshMaterial3d(handle));
            }
            commands.entity(root).add_child(child);
        }
    }

}

pub fn rotate_display_model(
    mut display_models: Query<&mut Transform, With<DisplayRoot>>,
    time: Res<Time>,
) {
    for mut trans in display_models.iter_mut() {
        //trans.rotate_x(1.0 * time.delta_seconds());
        trans.rotate_y(0.55 * time.delta_secs());
    }
}

pub fn manage_display_platform_visibility(
    display_models: Query<&DisplayRoot>,
    mut display_platforms: Query<&mut Visibility, With<DisplayModelStaging>>,
    commands: Commands,
) {
    if display_models.iter().len() <= 0 {
        for mut vis in display_platforms.iter_mut() {
            *vis = Visibility::Hidden
        }
    } else {
        for mut vis in display_platforms.iter_mut() {
            *vis = Visibility::Inherited
        }
    }
}



// /// helper function for displaying display models
// pub fn display_model(
//     //commands: &'a mut Commands,
//     //neon_materials: &'a mut ResMut<Assets<NeonMaterial>>,
//     //meshes: &'a mut ResMut<Assets<Mesh>>,
//     mut display_model: ResMut<DisplayModel>,
//     root: Entity,
// ) {
//     display_model.0 = Some(root)
//     // commands.spawn((
//     //     //Mesh3d(mesh),
//     //     Transform::from_translation(DISPLAY_MODEL_TRANSLATION),
//     //     MeshMaterial3d(neon_materials.add(LinearRgba::BLUE)),
//     //     RenderLayers::layer(1),
//     //     Name::new("showcase model"),
//     //     DisplayModel,
//     // ));
// }
