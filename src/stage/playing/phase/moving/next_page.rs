use bevy::prelude::{Event, EventReader, NextState, ResMut};

use crate::stage::state::StageState;

#[derive(Event, Debug, Copy, Clone, PartialEq)]
pub struct NextPageEvent;


pub fn next_page_event(
    mut state: ResMut<NextState<StageState>>,
    mut er: EventReader<NextPageEvent>,
) {
    if er.iter().next().is_some() {
        state.set(StageState::NextPage);
    }
}