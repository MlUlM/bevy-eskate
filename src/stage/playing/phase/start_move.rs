use bevy::app::{App, Plugin, Update};
use bevy::math::Vec3Swizzles;
use bevy::prelude::{Component, Entity, Event, EventReader, EventWriter, GlobalTransform, in_state, IntoSystemConfigs, Query, Res, Transform, With, Without};
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


#[derive(Event, Copy, Clone)]
pub struct StartMoveDownEvent(pub f32);


#[derive(Debug, Default, Copy, Clone, Hash, Eq, PartialEq)]
pub struct PlayingStartMovePlugin;

impl Plugin for PlayingStartMovePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_undo_event::<UndoPlayerEvent>()
            .add_event::<StartMoveEvent>()
            .add_event::<StartMoveDownEvent>()
            .add_systems(Update, (
                start_move,
                start_move_down_event_system,
                undo_player_pos_event_system
            ).run_if(in_state(GameState::Stage)));
    }
}


#[derive(Event, Copy, Clone, PartialEq, Debug)]
pub struct StartMoveEvent(pub MoveDirection);

fn start_move(
    mut scheduler: UndoScheduler<UndoPlayerEvent>,
    mut er: EventReader<StartMoveEvent>,
    mut ew: EventWriter<MoveEvent>,
    mut collides: Query<(Entity,  &mut GlobalTransform, &PageIndex), (Without<Movable>, With<PageIndex>, With<GimmickCollide>)>,
    players: Query<(&Transform, &GlobalTransform), With<Movable>>,
    page_index: Res<PageIndex>,
) {
    for StartMoveEvent(move_direction) in er.into_iter().copied() {
        for (pt, pgt) in players.iter() {
            if let Some((col_entity, _, _)) = collides
                .iter_mut()
                .filter(|(_, _, idx)| *page_index == **idx)
                .filter(|(_, transform, _)| {
                    filter_move_direction(pgt, transform, &move_direction)
                })
                .sorted_by(|(_, prev, _), (_, next, _)| {
                    distance(pgt, prev, &move_direction).partial_cmp(&distance(pgt, next, &move_direction)).unwrap()
                        .then(prev.translation().z.partial_cmp(&next.translation().y).unwrap())
                })
                .next()
            {
                scheduler.reserve(UndoPlayerEvent(*pt));
                ew.send(MoveEvent::new(move_direction, col_entity));
            }
        }
    }
}


fn start_move_down_event_system(
    mut start_move_down_reader: EventReader<StartMoveDownEvent>,
    mut start_move_writer: EventWriter<StartMoveEvent>,
    mut move_writer: EventWriter<MoveEvent>,
    page_index: Res<PageIndex>,
    player: Query<&Transform, With<Player>>,
    collides: Query<(Entity, &Transform, &PageIndex), (Without<Player>, With<PageIndex>, With<GimmickCollide>)>,
) {
    for StartMoveDownEvent(z) in start_move_down_reader.iter().copied() {
        let pt = player.single();
        if let Some((ce, _, _)) = collides
            .iter()
            .filter(|(_, _, idx)| **idx == *page_index)
            .filter(|(_, ct, _)| ct.translation.xy().abs_diff_eq(pt.translation.xy(), 0.1))
            .filter(|(_, ct, _)| ct.translation.z < z)
            .sorted_by(|(_, prev, _), (_, next, _)| prev.translation.z.partial_cmp(&next.translation.z).unwrap())
            .last() {
            move_writer.send(MoveEvent::new(MoveDirection::from_transform(pt), ce));
        } else {
            start_move_writer.send(StartMoveEvent(MoveDirection::from_transform(pt)));
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
    player_transform: &GlobalTransform,
    controller_transform: &GlobalTransform,
    direction: &MoveDirection,
) -> bool {
    match direction {
        MoveDirection::Left => controller_transform.translation().x < player_transform.translation().x && controller_transform.translation().y == player_transform.translation().y,
        MoveDirection::Right => player_transform.translation().x < controller_transform.translation().x && controller_transform.translation().y == player_transform.translation().y,
        MoveDirection::Up => player_transform.translation().y < controller_transform.translation().y && controller_transform.translation().x == player_transform.translation().x,
        MoveDirection::Down => controller_transform.translation().y < player_transform.translation().y && controller_transform.translation().x == player_transform.translation().x,
    }
}


fn distance(
    player_transform: &GlobalTransform,
    controller_transform: &GlobalTransform,
    direction: &MoveDirection,
) -> f32 {
    match direction {
        MoveDirection::Left | MoveDirection::Right => {
            (controller_transform.translation().x - player_transform.translation().x).abs()
        }
        MoveDirection::Up | MoveDirection::Down => {
            (player_transform.translation().y - controller_transform.translation().y).abs()
        }
    }
}