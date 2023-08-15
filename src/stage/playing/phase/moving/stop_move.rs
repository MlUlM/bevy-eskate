use bevy::prelude::{Event, EventReader, NextState, ResMut};

use crate::gama_state::GameState;

#[derive(Event, Copy, Clone, Debug)]
pub struct StopMoveEvent;


pub fn stop_move_event_system(
    mut er: EventReader<StopMoveEvent>,
    mut state: ResMut<NextState<GameState>>,
) {
    for _ in er.iter() {
        println!("stop move");
        state.set(GameState::StagePlayingIdle);
    }
}