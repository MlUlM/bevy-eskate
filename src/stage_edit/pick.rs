use bevy::prelude::*;
use bevy_undo::prelude::EntityCommandsOnUndoExt;

use crate::button::SpriteInteraction;
use crate::gama_state::GameState;
use crate::playing::gimmick::asset::GimmickAssets;
use crate::playing::gimmick::{Floor, Gimmick};
use crate::playing::PageIndex;
use crate::stage_edit::{front, gimmick_iem_sprite_bundle, StageEditState};
use crate::stage_edit::idle::OnPick;

#[derive(Debug, Default, Copy, Clone, Hash, Eq, PartialEq)]
pub struct StageEditPickedPlugin;


impl Plugin for StageEditPickedPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, update.run_if(in_state(GameState::StageEdit).and_then(in_state(StageEditState::PickItem))));
    }
}


fn update(
    assets: Res<GimmickAssets>,
    page_index: Res<PageIndex>,
    mut state: ResMut<NextState<StageEditState>>,
    mut commands: Commands,
    item: Query<(Entity, &OnPick), With<OnPick>>,
    floors: Query<(&Transform, &SpriteInteraction), (With<SpriteInteraction>, With<Floor>)>,
) {
    for (transform, interaction, ) in floors.iter() {
        if interaction.is_clicked() {
            let (on_pick_entity, OnPick(tag)) = item.single();
            commands.entity(on_pick_entity).remove::<OnPick>();

            commands
                .spawn(gimmick_iem_sprite_bundle(front(transform.translation), tag.image(&assets)))
                .insert(Gimmick(*tag))
                .insert(PageIndex::new(page_index.0))
                .on_undo(|cmd, entity| {
                    cmd.entity(entity).despawn();
                });
            state.set(StageEditState::Idle);
            return;
        }
    }
}


#[cfg(test)]
mod tests {
    use bevy::app::App;
    use bevy::prelude::NextState;

    use crate::stage_edit::StageEditState;

    #[test]
    fn drop_item() {
        let mut app = App::new();
        app.add_state::<StageEditState>();
        app
            .world
            .resource_mut::<NextState<StageEditState>>()
            .set(StageEditState::PickItem);

        assert_eq!(1, 1);
    }
}