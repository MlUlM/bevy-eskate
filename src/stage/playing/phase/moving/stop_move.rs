use bevy::prelude::{Event, EventReader, NextState, ResMut};
use bevy_undo2::prelude::{UndoScheduler};
use crate::stage::playing::phase::idle::UndoPlayerIdleEvent;

use crate::stage::state::StageState;

#[derive(Event, Copy, Clone, Debug)]
pub struct StopMoveEvent;


pub fn stop_move_event_system(
    mut state: ResMut<NextState<StageState>>,
    mut er: EventReader<StopMoveEvent>,
    mut scheduler: UndoScheduler<UndoPlayerIdleEvent>,
) {
    for _ in er.iter() {
        scheduler.reserve(UndoPlayerIdleEvent);
        scheduler.register_all_reserved();
        state.set(StageState::Idle);

        println!("stop move");
    }
}