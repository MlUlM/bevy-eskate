use bevy::app::{App, Plugin, Update};
use bevy::ecs::schedule::Condition;
use bevy::math::Vec3;
use bevy::prelude::{any_with_component, Commands, Entity, in_state, IntoSystemConfigs, Query, Res, Transform, With, Without};
use bevy_trait_query::imports::Component;
use bevy_trait_query::One;
use bevy_undo::prelude::EntityCommandsOnUndoExt;
use itertools::Itertools;

use crate::gama_state::GameState;
use crate::page::page_index::PageIndex;
use crate::stage::playing::collide::GimmickCollide;
use crate::stage::playing::gimmick::{move_linear, undo_move_linear};
use crate::stage::playing::gimmick::player::{Movable, Player};
use crate::stage::playing::move_direction::MoveDirection;
use crate::stage::playing::move_position::MovePosition;
use crate::stage::playing::phase::PlayingPhase;
use crate::stage::status::StageStatus;

#[derive(Debug, Default, Copy, Clone, Hash, Eq, PartialEq)]
pub struct PlayingStartMovePlugin;

impl Plugin for PlayingStartMovePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                start_move.run_if(in_state(GameState::Stage).and_then(run_if_start_move)),
                collide_system.run_if(in_state(GameState::Stage).and_then(any_with_component::<OnCollide>()))
            ));
    }
}

#[inline]
fn run_if_start_move(
    phase: Res<StageStatus>,
) -> bool {
    matches!(*phase, StageStatus::Playing(PlayingPhase::StartMove(_)))
}


#[derive(Component, Copy, Clone, PartialEq)]
struct OnCollide {
    player_previous_pos: Vec3,
}

fn start_move(
    phase: Res<StageStatus>,
    page_index: Res<PageIndex>,
    mut commands: Commands,
    mut players: Query<(Entity, &mut Transform), With<Movable>>,
    mut collides: Query<(Entity, One<&dyn MovePosition>, &mut Transform, &PageIndex), (Without<Movable>, With<PageIndex>)>,
) {
    let move_direction = phase.require_start_move();

    for (player, player_transform) in players.iter_mut() {
        if let Some((col_entity, collide, collide_transform, _)) = collides
            .iter_mut()
            .filter(|(_, _, _, idx)| *page_index == **idx)
            .filter(|(_, _, transform, _)| {
                filter_move_direction(&player_transform, transform, &move_direction)
            })
            .sorted_by(|(_, _, prev, _), (_, _, next, _)| {
                distance(&player_transform, prev, &move_direction).partial_cmp(&distance(&player_transform, next, &move_direction)).unwrap()
            })
            .next()
        {
            commands.insert_resource(StageStatus::playing_moving());
            let start = player_transform.translation;
            let end = collide.move_pos(collide_transform.translation, move_direction);

            move_linear(&mut commands.entity(player), start, end, move |cmd| {
                cmd
                    .commands()
                    .entity(col_entity)
                    .insert(OnCollide { player_previous_pos: start });
            });
        }
    }
}


fn collide_system(
    mut commands: Commands,
    player: Query<(Entity, &Transform), With<Player>>,
    collide: Query<(Entity, &OnCollide, &GimmickCollide), With<GimmickCollide>>,
) {
    let (col_entity, OnCollide { player_previous_pos }, collide) = collide.single();
    commands.entity(col_entity).remove::<OnCollide>();

    let player_previous_pos = *player_previous_pos;

    let (player, player_transform) = player.single();
    let player_pos = player_transform.translation;

    let undo = |commands: &mut Commands| {
        commands
            .entity(player)
            .on_undo_with_entity_commands(move |cmd| {
                undo_move_linear(cmd, player_pos, player_previous_pos);
            });
    };

    match collide {
        GimmickCollide::StopMove => {
            commands.insert_resource(StageStatus::playing_idle());
            undo(&mut commands);
        }
        GimmickCollide::NextPage => {
            commands.insert_resource(StageStatus::playing_next_page());
            commands
                .entity(player)
                .on_undo_with_entity_commands(move |cmd| {
                    cmd.commands().insert_resource(StageStatus::playing_previous_page());
                    undo_move_linear(cmd, player_pos, player_previous_pos);
                });
        }

        GimmickCollide::Goal => {
            commands.insert_resource(StageStatus::playing_goaled());
        }
        _ => {}
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