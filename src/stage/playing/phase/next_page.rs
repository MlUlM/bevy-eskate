use bevy::app::{App, Plugin, Update};
use bevy::prelude::{Commands, Component, Condition, Entity, in_state, IntoSystemConfigs, Query, resource_exists_and_equals, With};

use crate::gama_state::GameState;
use crate::page::page_index::PageIndex;
use crate::page::page_param::PageParams;
use crate::stage::playing::gimmick::GimmickItemSpawned;
use crate::stage::status::StageStatus;

#[derive(Component, Copy, Clone, Debug, Eq, PartialEq, Hash, Default)]
pub struct NextPage;


#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug, Default)]
pub struct PlayingNextPagePlugin;


impl Plugin for PlayingNextPagePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, next_page_system)
            .add_systems(Update, previous_page);
    }
}


fn next_page_system(
    mut commands: Commands,
    mut page_params: PageParams,
    items: Query<(Entity, &mut PageIndex), (With<GimmickItemSpawned>, With<PageIndex>)>,
) {
    commands.insert_resource(StageStatus::playing_idle());
    let next_page = page_params.next_page();

    update_items_page_index(next_page, items);
}


fn previous_page(
    mut commands: Commands,
    mut page_params: PageParams,
    items: Query<(Entity, &mut PageIndex), (With<GimmickItemSpawned>, With<PageIndex>)>,
) {
    commands.insert_resource(StageStatus::playing_idle());
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