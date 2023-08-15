use bevy::prelude::{Event, EventReader, NextState, ResMut};

use crate::gama_state::GameState;

#[derive(Event, Copy, Clone, Hash, Eq, PartialEq, Debug, Default)]
pub struct GoaledEvent;


pub fn goaled_event_system(
    mut state: ResMut<NextState<GameState>>,
    mut er: EventReader<GoaledEvent>,
) {
    if er.iter().next().is_some() {
        state.set(GameState::StageSelect);
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn page_move_spawned_items() {}
}