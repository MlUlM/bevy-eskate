use bevy::prelude::{Event, EventReader, NextState, ResMut};
use bevy_undo2::prelude::UndoReserveCommitter;

use crate::stage::state::StageState;

#[derive(Event, Copy, Clone, Debug)]
pub struct StopMoveEvent;


pub fn stop_move_event_system(
    mut state: ResMut<NextState<StageState>>,
    mut er: EventReader<StopMoveEvent>,
    mut committer: UndoReserveCommitter,
) {
    for _ in er.iter() {
        committer.commit();
        state.set(StageState::Idle);
        println!("stop move");
    }
}