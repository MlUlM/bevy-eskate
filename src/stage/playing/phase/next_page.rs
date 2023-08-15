use bevy::app::{App, Plugin};
use bevy::prelude::{Component, Entity, NextState, OnEnter, Query, ResMut, With};

use crate::page::page_index::PageIndex;
use crate::page::page_param::PageParams;
use crate::stage::playing::gimmick::GimmickItemSpawned;
use crate::stage::state::StageState;

#[derive(Component, Copy, Clone, Debug, Eq, PartialEq, Hash, Default)]
pub struct NextPage;


#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug, Default)]
pub struct PlayingNextPagePlugin;


impl Plugin for PlayingNextPagePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(StageState::NextPage), next_page_system);
    }
}


fn next_page_system(
    mut state: ResMut<NextState<StageState>>,
    mut page_params: PageParams,
    items: Query<(Entity, &mut PageIndex), (With<GimmickItemSpawned>, With<PageIndex>)>,
) {
    let next_page = page_params.next_page();
    update_items_page_index(next_page, items);
    state.set(StageState::Idle);
}


fn previous_page(
    mut page_params: PageParams,
    items: Query<(Entity, &mut PageIndex), (With<GimmickItemSpawned>, With<PageIndex>)>,
) {
    let previous_page = page_params.previous_page();
    update_items_page_index(previous_page, items);
}


fn update_items_page_index(
    next_page_index: PageIndex,
    mut items: Query<(Entity, &mut PageIndex), (With<GimmickItemSpawned>, With<PageIndex>)>,
) {
    for (_, mut page_index) in items.iter_mut() {
        *page_index = next_page_index;
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn page_move_spawned_items() {}
}