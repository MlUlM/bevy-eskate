use bevy::app::{App, Plugin, Update};
use bevy::prelude::{Component, Condition, in_state, IntoSystemConfigs, NextState, Res, ResMut};

use crate::gama_state::GameState;
use crate::stage::playing::phase::PlayingPhase;
use crate::stage::status::StageStatus;

#[derive(Component, Copy, Clone, Debug, Eq, PartialEq, Hash, Default)]
pub struct NextPage;


#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug, Default)]
pub struct PlayingGoaledPlugin;


impl Plugin for PlayingGoaledPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (goaled)
            .run_if(in_state(GameState::Stage).and_then(run_if_goaled_phase)),
        );
    }
}


#[inline]
fn run_if_goaled_phase(
    phase: Res<StageStatus>,
) -> bool {
    matches!(*phase, StageStatus::Playing(PlayingPhase::Goaled))
}


fn goaled(
    mut state: ResMut<NextState<GameState>>
) {
    state.set(GameState::StageSelect);
}


#[cfg(test)]
mod tests {
    #[test]
    fn page_move_spawned_items() {}
}