use bevy::prelude::*;

use crate::gama_state::GameState;
use crate::gimmick::{Floor, Stage};
use crate::stage_creator::idle::OnPick;
use crate::stage_creator::StageCreatorState;

#[derive(Debug, Default, Copy, Clone, Hash, Eq, PartialEq)]
pub struct StageCreatorPickedPlugin;


impl Plugin for StageCreatorPickedPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, update.run_if(in_state(GameState::StageCreator).and_then(in_state(StageCreatorState::PickItem))));
    }
}


fn update(
    asset: Res<AssetServer>,
    mut state: ResMut<NextState<StageCreatorState>>,
    mut commands: Commands,
    stage: Query<Entity, With<Stage>>,
    item: Query<&OnPick, With<OnPick>>,
    floors: Query<(&Transform, &Interaction), (With<Button>, With<Floor>)>,
) {
    for (transform, interaction, ) in floors.iter() {
        if interaction == &Interaction::Pressed {
            let OnPick(tag) = item.single();

            let cell = commands
                .spawn(SpriteBundle {
                    transform: Transform::from_xyz(transform.translation.x, transform.translation.y, 1.),
                    texture: tag.load(&asset),
                    ..default()
                })
                .id();


            println!("{tag:?}");
            state.set(StageCreatorState::Idle);
            return;
        }
    }
}


#[cfg(test)]
mod tests {
    use bevy::app::App;
    use bevy::prelude::NextState;

    use crate::stage_creator::StageCreatorState;

    #[test]
    fn drop_item() {
        let mut app = App::new();
        app.add_state::<StageCreatorState>();
        app
            .world
            .resource_mut::<NextState<StageCreatorState>>()
            .set(StageCreatorState::PickItem);

        assert_eq!(1, 1);
    }
}