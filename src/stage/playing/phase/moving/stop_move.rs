use bevy::prelude::{Event, EventReader, NextState, ResMut};
use crate::stage::state::StageState;

#[derive(Event, Copy, Clone, Debug)]
pub struct StopMoveEvent;


pub fn stop_move_event_system(
    mut state: ResMut<NextState<StageState>>,
    mut er: EventReader<StopMoveEvent>,
) {
    for _ in er.iter() {
        state.set(StageState::Idle);
        println!("stop move");
    }
}