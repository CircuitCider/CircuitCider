use crate::components::{Bullet, CollisionDamage, Health, Velocity, Enemy, Weapon, Animations, Graph};
use bevy::prelude::*;
use bevy_rapier3d::{plugin::DefaultRapierContext, prelude::{RapierContextColliders, RapierContextSimulation}, rapier::prelude::ColliderFlags};
use bevy_serialization_extras::prelude::colliders::ColliderFlag;
use bevy::animation::{animate_targets, RepeatAnimation};
use std::time::Duration;

pub fn apply_collision_damage(
    collision_damage_query: Query<(Entity, &CollisionDamage, &ColliderFlag)>,
    health_query: Query<&mut Health>,
<<<<<<< Updated upstream
    // rapier_context: Res<RapierContext>,
=======
    rapier_context_colliders: Single<&RapierContextColliders>,
    rapier_context_simulation: Single<&RapierContextSimulation, With <DefaultRapierContext>>,
>>>>>>> Stashed changes
    name_query: Query<&Bullet>,
    commands: Commands,
) {
    //   for (e, damage) in collision_damage_query.iter() {
    //         if rapier_context_simulation.
    //         intersection_pairs_with(rapier_context_colliders, e)
    //         .collect::<Vec<_>>()
    //         .len() <= 0
    //         {
    //             let e_target = e;
    //             let Ok(mut health) = health_query.get_mut(e_target) else {
    //                 return;
    //             };

    //             health.hp -= damage.0;
    //             if name_query.get(e_target).is_ok() {
    //                 commands.entity(e_target).despawn_recursive();
    //             }
    //         }
    //     }
<<<<<<< Updated upstream
} // commented out till further notice
=======
}

>>>>>>> Stashed changes
pub fn update_position(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.value * time.delta_secs();
    }
}

pub fn mace_light_animation(
    mut commands: Commands,
    mut players_query: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    //create animation graph and load clips
        //animation graph
        let mut graph = AnimationGraph::new();
        let animation_nodes: Vec<AnimationNodeIndex> = graph
            .add_clips(
                [
                    GltfAssetLabel::Animation(0).from_asset("mace.glb#Scene0"),
                ]
                .into_iter()
                .map(|path| asset_server.load(path)),
                1.0,
                graph.root,
            )
            .collect();
        //mace animation
        let graph_handle = graphs.add(graph);
        commands.insert_resource(Animations {
            animations: animation_nodes.clone(),
            graph: graph_handle.clone(),
        });
    //assign animation and graph to mace
    for (entity, mut player) in &mut players_query {
        let mut transitions = AnimationTransitions::new();
        transitions.play(&mut player, animation_nodes[0], Duration::ZERO);
        //don't change the n umber it crashes the application :(
        commands
            .entity(entity)
            .insert(Graph(graph_handle.clone()))
            .insert(transitions);
    }
}

//on left mouse click, decrease enemy health
pub fn light_attack(
    mut health_query: Query<&mut Health, With<Enemy>>,
    buttons: Res<ButtonInput<MouseButton>>
) {
    let mut health = health_query.single_mut();
    if buttons.just_pressed(MouseButton::Left) {
        //decrement enemy health
    }
}

//on mouse left click, play weapon animation once
pub fn mouse_animation_control(
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut animation_players: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
) {
    for (mut player, mut transitions) in &mut animation_players {
        let Some((&playing_animation_index, _)) = player.playing_animations().next() else {
            continue;        
    };

    if mouse_input.just_pressed(MouseButton::Left) {
        let playing_animation = player.animation_mut(playing_animation_index).unwrap();
        playing_animation
            .set_repeat(RepeatAnimation::Count(1))
            .replay();
    }
  }
}