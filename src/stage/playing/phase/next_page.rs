use bevy::app::{App, Plugin, Update};
use bevy::hierarchy::BuildChildren;
use bevy::log::debug;
use bevy::prelude::{Commands, Component, Entity, Event, EventReader, NextState, OnEnter, Query, ResMut, With, Without};
use bevy_undo2::prelude::{AppUndoEx, UndoScheduler};

use crate::page::page_index::PageIndex;
use crate::page::page_param::PageParams;
use crate::stage::playing::gimmick::GimmickItemSpawned;
use crate::stage::state::StageState;
use crate::stage_edit::page::Field;

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
    commands: Commands,
    fields: Query<(Entity, &PageIndex), With<Field>>,
    items: Query<(Entity, &mut PageIndex), (With<GimmickItemSpawned>, With<PageIndex>, Without<Field>)>,
) {
    let next_page = page_params.next_page();
    debug!("next page");
    scheduler.reserve_default();
    scheduler.reserve_commit();
    update_items_page_index(commands, items, next_page, fields);
    state.set(StageState::Idle);
}


fn undo_next_page_event_system(
    mut er: EventReader<UndoNextPageEvent>,
    mut page_params: PageParams,
    commands: Commands,
    fields: Query<(Entity, &PageIndex), With<Field>>,
    items: Query<(Entity, &mut PageIndex), (With<GimmickItemSpawned>, With<PageIndex>, Without<Field>)>,
) {
    if er.iter().next().is_some() {
        debug!("undo: next page");
        let previous_page = page_params.previous_page();
        update_items_page_index(commands, items, previous_page, fields);
    }
}


fn update_items_page_index(
    mut commands: Commands,
    mut items: Query<(Entity, &mut PageIndex), (With<GimmickItemSpawned>, With<PageIndex>, Without<Field>)>,
    next_page_index: PageIndex,
    fields: Query<(Entity, &PageIndex), With<Field>>,
) {
    let (field, _) = fields.iter().find(|(_, idx)| **idx == next_page_index).unwrap();
    for (item, mut page_index) in items.iter_mut() {
        *page_index = next_page_index;
        commands.entity(field).add_child(item);
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn page_move_spawned_items() {}
}