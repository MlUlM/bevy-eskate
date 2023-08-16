use bevy::app::{App, Plugin, Update};
use bevy::prelude::{Component, Entity, Event, EventReader, EventWriter, in_state, IntoSystemConfigs, Query, Res, Transform, With, Without};
use bevy_undo2::prelude::{AppUndoEx, UndoScheduler};
use itertools::Itertools;

use crate::gama_state::GameState;
use crate::page::page_index::PageIndex;
use crate::stage::playing::collide::GimmickCollide;
use crate::stage::playing::gimmick::player::{Movable, Player};
use crate::stage::playing::move_direction::MoveDirection;
use crate::stage::playing::phase::moving::MoveEvent;

#[derive(Event, Copy, Clone, Component)]
pub struct UndoPlayerEvent(Transform);

impl UndoPlayerEvent {
    #[inline]
    pub fn new(transform: Transform) -> Self {
        Self(transform)
    }
}

#[derive(Debug, Default, Copy, Clone, Hash, Eq, PartialEq)]
pub struct PlayingStartMovePlugin;

impl Plugin for PlayingStartMovePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_undo_event::<UndoPlayerEvent>()
            .add_systems(Update, (
                start_move,
                undo_player_pos_event_system
            ).run_if(in_state(GameState::Stage)));
    }
}


#[derive(Event, Copy, Clone, PartialEq, Debug)]
pub struct StartMoveEvent(pub MoveDirection);

fn start_move(
    mut er: EventReader<StartMoveEvent>,
    mut ew: EventWriter<MoveEvent>,
    mut scheduler: UndoScheduler<UndoPlayerEvent>,
    mut collides: Query<(Entity, &mut Transform, &PageIndex), (Without<Movable>, With<PageIndex>, With<GimmickCollide>)>,
    players: Query<&Transform, With<Movable>>,
    page_index: Res<PageIndex>,
) {
    for StartMoveEvent(move_direction) in er.into_iter().copied() {
        println!("start_move {move_direction:?}");
        for player_transform in players.iter() {
            if let Some((col_entity, _, _)) = collides
                .iter_mut()
                .filter(|(_, _, idx)| *page_index == **idx)
                .filter(|(_, transform, _)| {
                    filter_move_direction(player_transform, transform, &move_direction)
                })
                .sorted_by(|(_, prev, _), (_, next, _)| {
                    distance(player_transform, prev, &move_direction).partial_cmp(&distance(player_transform, next, &move_direction)).unwrap()
                })
                .next()
            {
                scheduler.reserve(UndoPlayerEvent(*player_transform));
                ew.send(MoveEvent::new(move_direction, col_entity));
            }
        }
    }
}


fn undo_player_pos_event_system(
    mut er: EventReader<UndoPlayerEvent>,
    mut player: Query<&mut Transform, With<Player>>,
) {
    for e in er.iter() {
        *player.single_mut() = e.0;
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