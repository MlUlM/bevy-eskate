use bevy::prelude::{Commands, Component, Entity, Query, With};
use crate::gimmick::GimmickItemSpawned;
use crate::playing::PageIndex;

#[derive(Component, Copy, Clone, Debug)]
pub struct NextPage;


pub fn update_fall_down(
    mut commands: Commands,
    mut gimmicks: Query<(Entity, &mut PageIndex), (With<GimmickItemSpawned>, With<PageIndex>)>,
    status: Query<Entity, With<NextPage>>,
) {
}



#[cfg(test)]
mod tests{
    use bevy::prelude::system_adapter::new;

    #[test]
    fn page_move_spawned_items(){

    }
}