use bevy::app::{App, Plugin, Update};
use bevy::ecs::system::SystemParam;
use bevy::prelude::{Commands, Component, Event, EventReader, EventWriter, in_state, IntoSystem, IntoSystemConfigs, NextState, Query, ResMut, Transform, With, Without};
use bevy_trait_query::imports::Entity;
use bevy_trait_query::One;
use bevy_tweening::TweenCompleted;

use crate::gama_state::GameState;
use crate::page::page_index::PageIndex;
use crate::stage::playing::collide::GimmickCollide;
use crate::stage::playing::gimmick::move_linear;
use crate::stage::playing::gimmick::player::Movable;
use crate::stage::playing::move_direction::MoveDirection;
use crate::stage::playing::move_position::MovePosition;
use crate::stage::playing::phase::moving::stop_move::{stop_move_event_system, StopMoveEvent};
use crate::stage::playing::phase::moving::turn::{turn_completed, turn_event_system, turn_pipe_system, TurnEvent};

pub mod stop_move;
pub mod turn;

#[derive(Event, Copy, Clone, Eq, PartialEq)]
pub struct MoveEvent {
    move_direction: MoveDirection,
    col_entity: Entity,
}

impl MoveEvent {
    #[inline]
    pub const fn new(direction: MoveDirection, col_entity: Entity) -> Self {
        Self {
            move_direction: direction,
            col_entity,
        }
    }
}


#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Component)]
pub struct CollisionTarget;


#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct PlayingMovingPlugin;


impl Plugin for PlayingMovingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<MoveDoneEvent>()
            .add_event::<TurnEvent>()
            .add_systems(Update, move_event_system)
            .add_systems(Update, (
                move_done_system,
                collide_system,
                stop_move_event_system,
                turn_event_system.pipe(turn_pipe_system),
                turn_completed
            ).run_if(in_state(GameState::StagePlayingMove)));
    }
}

fn move_event_system(
    mut state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    mut er: EventReader<MoveEvent>,
    mut players: Query<(Entity, &mut Transform), With<Movable>>,
    mut collides: Query<(Entity, &mut Transform, One<&dyn MovePosition>), (Without<Movable>, With<PageIndex>)>,
) {
    for MoveEvent { move_direction, col_entity } in er.iter().copied() {
        let Some((ce, ct, move_position)) = collides.get_mut(col_entity).ok() else { continue; };

        for (pe, mut pt) in players.iter_mut() {
            let start = pt.translation;
            let end = move_position.move_pos(ct.translation, move_direction);
            move_linear(&mut commands.entity(pe), &mut pt, end, move_direction);
            commands.entity(ce).insert(CollisionTarget);
            state.set(GameState::StagePlayingMove);
        }
    }
}


#[derive(Event, Copy, Clone, Debug, Eq, PartialEq)]
struct MoveDoneEvent(Entity);


fn move_done_system(
    mut commands: Commands,
    mut er: EventReader<TweenCompleted>,
    mut ew: EventWriter<MoveDoneEvent>,
    col: Query<Entity, With<CollisionTarget>>,
) {
    for _ in er.iter().filter(|e| e.user_data == 1) {
        println!("move done");
        let ce = col.single();
        commands.entity(ce).remove::<CollisionTarget>();
        ew.send(MoveDoneEvent(ce));
    }
}


#[derive(SystemParam)]
struct CollideWriters<'w> {
    stop_move: EventWriter<'w, StopMoveEvent>,
    turn: EventWriter<'w, TurnEvent>,
}


impl<'w> CollideWriters<'w> {
    #[inline]
    pub fn stop_move(&mut self) {
        self.stop_move.send(StopMoveEvent);
    }

    #[inline]
    pub fn turn(&mut self, turn_entity: Entity) {
        self.turn.send(TurnEvent(turn_entity));
    }
}


fn collide_system(
    mut collide_writers: CollideWriters,
    mut er: EventReader<MoveDoneEvent>,
    cols: Query<&GimmickCollide>,
) {
    for MoveDoneEvent(ce) in er.iter().copied() {
        println!("collide_system");
        let Some(collide) = cols.get(ce).ok() else { continue; };
        match collide {
            GimmickCollide::StopMove => {
                collide_writers.stop_move();
            }
            GimmickCollide::Turn => {
                collide_writers.turn(ce);
            }
            _ => {}
        }
    }
}
