use bevy::app::{App, Plugin, Update};
use bevy::prelude::{Commands, Component, Condition, Entity, in_state, IntoSystemConfigs, Query, Res, With};

use crate::gama_state::GameState;
use crate::page::page_index::PageIndex;
use crate::page::page_param::PageParams;
use crate::stage::playing::gimmick::GimmickItemSpawned;
use crate::stage::playing::phase::PlayingPhase;
use crate::stage::status::StageStatus;

#[derive(Component, Copy, Clone, Debug, Eq, PartialEq, Hash, Default)]
pub struct NextPage;


#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug, Default)]
pub struct PlayingNextPagePlugin;


impl Plugin for PlayingNextPagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (next_page)
            .run_if(in_state(GameState::Stage).and_then(run_if_next_page_phase)),
        );
    }
}


#[inline]
fn run_if_next_page_phase(
    phase: Res<StageStatus>,
) -> bool {
    matches!(*phase, StageStatus::Playing(PlayingPhase::NextPage))
}


fn next_page(
    mut commands: Commands,
    mut page_params: PageParams,
    mut items: Query<(Entity, &mut PageIndex), (With<GimmickItemSpawned>, With<PageIndex>)>,
) {
    commands.insert_resource(StageStatus::playing_idle());
    let next_page = page_params.next_page();

    for (_, mut page_index) in items.iter_mut() {
        *page_index = next_page;
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn page_move_spawned_items() {}
}