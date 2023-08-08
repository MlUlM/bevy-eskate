use bevy::prelude::Component;

#[derive(Component, Copy, Clone, Debug)]
pub struct NextPage;

//
// #[derive(Copy, Clone, Hash, Eq, PartialEq)]
// pub struct PlayingNextPagePlugin;
//
//
// impl Plugin for PlayingNextPagePlugin{
//     fn build(&self, app: &mut App) {
//         app.add_systems(Update, (update_fall_down)
//             .run_if(in_state(GameState::Playing))
//         )
//     }
// }
//
//
// pub fn update_fall_down(
//     mut commands: Commands,
//     mut gimmicks: Query<(Entity, &mut PageIndex), (With<GimmickItemSpawned>, With<PageIndex>)>,
//     status: Query<Entity, With<NextPage>>,
// ) {
// }


#[cfg(test)]
mod tests {
    #[test]
    fn page_move_spawned_items() {}
}