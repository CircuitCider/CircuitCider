pub fn shoot_ray_down_to_target(
    cursor_ray: Res<CursorRay>,
    // mouse: Res<ButtonInput<MouseButton>>,
    // mut raycast: Raycast,
    // mut primary_window: Query<&Window, With<PrimaryWindow>>,
    mut gizmos: Gizmos,
    //cameras: Query<(&GlobalTransform, &Camera)>,
    mut targets: Query<(&mut Transform, &Grabbed)>
) {
    if let Some(cursor_ray) = **cursor_ray {
            let mut averaged_translation = Vec3::ZERO;
            for (mut trans, ..) in targets.iter_mut(){
                averaged_translation += trans.translation;
                //trans.translation.y = cursor_ray.origin.y;
            }
            averaged_translation = averaged_translation / targets.iter().len() as f32;
            gizmos.line(averaged_translation, cursor_ray.origin, Color::PURPLE);
            
        }
}