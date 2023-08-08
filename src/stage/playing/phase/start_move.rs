use bevy::app::{App, Plugin, Update};
use bevy::ecs::schedule::Condition;
use bevy::prelude::{Commands, Entity, in_state, IntoSystemConfigs, Query, Res, Transform, With, Without};
use bevy_trait_query::One;
use itertools::Itertools;

use crate::gama_state::GameState;
use crate::page::page_index::PageIndex;
use crate::stage::playing::gimmick::GimmickCollide;
use crate::stage::playing::gimmick::player::Movable;
use crate::stage::playing::move_direction::MoveDirection;
use crate::stage::playing::phase::PlayingPhase;
use crate::stage::status::StageStatus;

#[derive(Debug, Default, Copy, Clone, Hash, Eq, PartialEq)]
pub struct PlayingStartMovePlugin;

impl Plugin for PlayingStartMovePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, start_move
                .run_if(in_state(GameState::Stage).and_then(run_if_start_move)),
            );
    }
}

#[inline]
fn run_if_start_move(
    phase: Res<StageStatus>,
) -> bool {
    matches!(*phase, StageStatus::Playing(PlayingPhase::StartMove(_)))
}


fn start_move(
    phase: Res<StageStatus>,
    page_index: Res<PageIndex>,
    mut commands: Commands,
    mut players: Query<(Entity, &mut Transform), With<Movable>>,
    mut collides: Query<(One<&dyn GimmickCollide>, &mut Transform, &PageIndex), (Without<Movable>, With<PageIndex>)>,
) {
    let move_direction = phase.require_start_move();

    for (player, mut player_transform) in players.iter_mut() {
        if let Some((collide, mut collide_transform, _)) = collides
            .iter_mut()
            .filter(|(_, _, idx)| *page_index == **idx)
            .filter(|(_, transform, _)| {
                filter_move_direction(&player_transform, transform, &move_direction)
            })
            .sorted_by(|(_, prev, _), (_, next, _)| {
                distance(&player_transform, prev, &move_direction).partial_cmp(&distance(&player_transform, next, &move_direction)).unwrap()
            })
            .next()
        {
            commands.insert_resource(StageStatus::playing_moving());
            println!("{collide_transform:?} player = {player_transform:?}");
            collide.move_player(
                &mut commands.get_entity(player).unwrap(),
                &mut collide_transform,
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