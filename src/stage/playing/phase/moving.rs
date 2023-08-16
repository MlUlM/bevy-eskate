use bevy::app::{App, Plugin, Update};
use bevy::ecs::system::SystemParam;
use bevy::math::Vec3Swizzles;
use bevy::prelude::{Commands, Component, Event, EventReader, EventWriter, in_state, IntoSystem, IntoSystemConfigs, NextState, Query, ResMut, Transform, With, Without};
use bevy_trait_query::imports::Entity;
use bevy_trait_query::One;
use bevy_tweening::TweenCompleted;

use crate::page::page_index::PageIndex;
use crate::stage::playing::collide::GimmickCollide;
use crate::stage::playing::gimmick::move_linear;
use crate::stage::playing::gimmick::player::Movable;
use crate::stage::playing::move_direction::MoveDirection;
use crate::stage::playing::move_position::MovePosition;
use crate::stage::playing::phase::moving::goaled::{goaled_event_system, GoaledEvent};
use crate::stage::playing::phase::moving::key::{KeyEvent, PlayingKeyPlugin};
use crate::stage::playing::phase::moving::next_page::{next_page_event, NextPageEvent};
use crate::stage::playing::phase::moving::stop_move::{stop_move_event_system, StopMoveEvent};
use crate::stage::playing::phase::moving::turn::{turn_completed, turn_event_system, turn_pipe_system, TurnEvent};
use crate::stage::state::StageState;

pub mod stop_move;
pub mod turn;
mod next_page;
pub mod goaled;
mod key;

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
            .add_plugins(
                PlayingKeyPlugin
            )
            .add_event::<CollisionEvent>()
            .add_event::<TurnEvent>()
            .add_event::<NextPageEvent>()
            .add_event::<GoaledEvent>()
            .add_systems(Update, move_event_system)
            .add_systems(Update, (
                move_done_system,
                collide_system,
                stop_move_event_system,
                turn_event_system.pipe(turn_pipe_system),
                turn_completed,
                next_page_event,
                goaled_event_system
            ).run_if(in_state(StageState::Moving)));
    }
}


fn move_event_system(
    mut state: ResMut<NextState<StageState>>,
    mut commands: Commands,
    mut er: EventReader<MoveEvent>,
    mut players: Query<(Entity, &mut Transform), With<Movable>>,
    mut collides: Query<(Entity, &mut Transform, One<&dyn MovePosition>), (Without<Movable>, With<PageIndex>)>,
) {
    for MoveEvent { move_direction, col_entity } in er.iter().copied() {
        let Some((ce, ct, move_position)) = collides.get_mut(col_entity).ok() else { continue; };

        for (pe, mut pt) in players.iter_mut() {
            let end = move_position.move_pos(ct.translation, move_direction);
            if pt.translation.xy().abs_diff_eq(end.xy(), 0.1) {
                continue;
            }

            move_linear(&mut commands.entity(pe), &mut pt, end, move_direction);
            commands.entity(ce).insert(CollisionTarget);
            state.set(StageState::Moving);
        }
    }
}


#[derive(Event, Copy, Clone, Debug, Eq, PartialEq)]
struct CollisionEvent(Entity);


fn move_done_system(
    mut commands: Commands,
    mut er: EventReader<TweenCompleted>,
    mut ew: EventWriter<CollisionEvent>,
    col: Query<Entity, With<CollisionTarget>>,
) {
    for _ in er.iter().filter(|e| e.user_data == 1) {
        println!("move done");
        let ce = col.single();
        commands.entity(ce).remove::<CollisionTarget>();
        ew.send(CollisionEvent(ce));
    }
}


#[derive(SystemParam)]
struct CollideWriters<'w> {
    stop_move: EventWriter<'w, StopMoveEvent>,
    turn: EventWriter<'w, TurnEvent>,
    next_page: EventWriter<'w, NextPageEvent>,
    goaled: EventWriter<'w, GoaledEvent>,
    key: EventWriter<'w, KeyEvent>,
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


    #[inline]
    pub fn next_page(&mut self) {
        self.next_page.send(NextPageEvent);
    }


    #[inline]
    pub fn goaled(&mut self) {
        self.goaled.send(GoaledEvent);
    }
}


fn collide_system(
    mut collide_writers: CollideWriters,
    mut er: EventReader<CollisionEvent>,
    cols: Query<&GimmickCollide>,
) {
    for CollisionEvent(ce) in er.iter().copied() {
        let Some(collide) = cols.get(ce).ok() else { continue; };
        println!("collide_system {collide:?}");

        match collide {
            GimmickCollide::StopMove => {
                collide_writers.stop_move();
            }
            GimmickCollide::Turn => {
                collide_writers.turn(ce);
            }
            GimmickCollide::NextPage => {
                collide_writers.next_page();
            }
            GimmickCollide::Goal => {
                collide_writers.goaled();
            }
            GimmickCollide::Key => {
                collide_writers.key.send(KeyEvent(ce));
            }
            _ => {}
        }
    }
}
