use bevy::app::{App, Plugin, Update};
use bevy::prelude::{Component, Entity, Event, EventReader, NextState, OnEnter, Query, ResMut, With};
use bevy_undo2::prelude::{AppUndoEx, UndoScheduler};

use crate::page::page_index::PageIndex;
use crate::page::page_param::PageParams;
use crate::stage::playing::gimmick::GimmickItemSpawned;
use crate::stage::state::StageState;

#[derive(Component, Copy, Clone, Debug, Eq, PartialEq, Hash, Default)]
pub struct NextPage;


#[derive(Event, Copy, Clone, Debug, Default)]
struct UndoNextPageEvent;


#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug, Default)]
pub struct PlayingNextPagePlugin;


impl Plugin for PlayingNextPagePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_undo_event::<UndoNextPageEvent>()
            .add_systems(OnEnter(StageState::NextPage), next_page_system)
            .add_systems(Update, undo_next_page_event_system);
    }
}


fn next_page_system(
    mut state: ResMut<NextState<StageState>>,
    mut scheduler: UndoScheduler<UndoNextPageEvent>,
    mut page_params: PageParams,
    items: Query<(Entity, &mut PageIndex), (With<GimmickItemSpawned>, With<PageIndex>)>,
) {
    let next_page = page_params.next_page();
    println!("next page");
    scheduler.reserve_default();
    scheduler.reserve_commit();
    update_items_page_index(next_page, items);
    state.set(StageState::Idle);
}


fn undo_next_page_event_system(
    mut er: EventReader<UndoNextPageEvent>,
    mut page_params: PageParams,
    items: Query<(Entity, &mut PageIndex), (With<GimmickItemSpawned>, With<PageIndex>)>,
) {
    if er.iter().next().is_some() {
        println!("undo: next page");
        let previous_page = page_params.previous_page();
        update_items_page_index(previous_page, items);
    }
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