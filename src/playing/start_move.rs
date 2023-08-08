use bevy::app::{App, Plugin, Update};
use bevy::prelude::{Commands, Entity, in_state, IntoSystemConfigs, Query, Res, Transform, With, Without};
use bevy_trait_query::One;
use bevy::ecs::schedule::Condition;
use itertools::Itertools;
use crate::gama_state::GameState;

use crate::playing::gimmick::GimmickCollide;
use crate::playing::gimmick::player::{Movable};
use crate::playing::move_direction::MoveDirection;
use crate::playing::phase::PlayingPhase;


#[derive(Debug, Default, Copy, Clone, Hash, Eq, PartialEq)]
pub struct PlayingStartMovePlugin;

impl Plugin for PlayingStartMovePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, start_move
                .run_if(in_state(GameState::Playing).and_then(run_if_start_move))
            );
    }
}

#[inline]
fn run_if_start_move(
    phase: Res<PlayingPhase>,
) -> bool {
    matches!(*phase, PlayingPhase::StartMove(_))
}


fn start_move(
    phase: Res<PlayingPhase>,
    mut commands: Commands,
    mut players: Query<(Entity, &mut Transform), With<Movable>>,
    mut collides: Query<(One<&dyn GimmickCollide>, &mut Transform), Without<Movable>>,
) {
    let move_direction = phase.require_start_move();

    for (player, mut player_transform) in players.iter_mut() {
        if let Some((collide, mut controller_transform)) = collides
            .iter_mut()
            .filter(|(_, transform)| {
                filter_move_direction(&player_transform, transform, &move_direction)
            })
            .sorted_by(|(_, prev), (_, next)| {
                distance(&player_transform, prev, &move_direction).partial_cmp(&distance(&player_transform, next, &move_direction)).unwrap()
            })
            .next()
        {
            commands.insert_resource(PlayingPhase::Moving);

            collide.move_player(
                &mut commands.get_entity(player).unwrap(),
                &mut controller_transform,
                &mut player_transform,
                &move_direction,
            );
        }
    }
}


fn filter_move_direction(
    player_transform: &Transform,
    controller_transform: &Transform,
    direction: &MoveDirection,
) -> bool {
    match direction {
        MoveDirection::Left => controller_transform.translation.x < player_transform.translation.x && controller_transform.translation.y == player_transform.translation.y,
        MoveDirection::Right => player_transform.translation.x < controller_transform.translation.x && controller_transform.translation.y == player_transform.translation.y,
        MoveDirection::Up => player_transform.translation.y < controller_transform.translation.y && controller_transform.translation.x == player_transform.translation.x,
        MoveDirection::Down => controller_transform.translation.y < player_transform.translation.y && controller_transform.translation.x == player_transform.translation.x,
    }
}


fn distance(
    player_transform: &Transform,
    controller_transform: &Transform,
    direction: &MoveDirection,
) -> f32 {
    match direction {
        MoveDirection::Left | MoveDirection::Right => {
            (controller_transform.translation.x - player_transform.translation.x).abs()
        }
        MoveDirection::Up | MoveDirection::Down => {
            (player_transform.translation.y - controller_transform.translation.y).abs()
        }
    }
}