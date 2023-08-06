use bevy::prelude::*;
use bevy_undo::prelude::EntityCommandsOnUndoExt;

use crate::button::SpriteInteraction;
use crate::gama_state::GameState;
use crate::gimmick::{Floor, Gimmick};
use crate::stage_creator::{front, gimmick_iem_sprite_bundle, StageCreatorState};
use crate::stage_creator::idle::OnPick;

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
    item: Query<(Entity, &OnPick), With<OnPick>>,
    floors: Query<(&Transform, &SpriteInteraction), (With<SpriteInteraction>, With<Floor>)>,
) {
    for (transform, interaction, ) in floors.iter() {
        if interaction.is_clicked() {
            let (on_pick_entity, OnPick(tag)) = item.single();
            commands.entity(on_pick_entity).remove::<OnPick>();

            commands
                .spawn(gimmick_iem_sprite_bundle(front(transform.translation), tag.load(&asset)))
                .insert(Gimmick(*tag))
                .on_undo(|cmd, entity| {
                    cmd.entity(entity).despawn();
                });
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