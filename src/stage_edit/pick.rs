use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy_undo::prelude::EntityCommandsOnUndoExt;

use crate::assets::gimmick::GimmickAssets;
use crate::button::SpriteInteraction;
use crate::gama_state::GameState;
use crate::page::page_index::PageIndex;
use crate::stage::playing::gimmick::{Floor, Gimmick};
use crate::stage_edit::{front, gimmick_iem_sprite_bundle, StageEditStatus};
use crate::stage_edit::idle::OnPick;

#[derive(Debug, Default, Copy, Clone, Hash, Eq, PartialEq)]
pub struct StageEditPickedPlugin;


impl Plugin for StageEditPickedPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, update
                .run_if(in_state(GameState::StageEdit)
                    .and_then(resource_exists_and_equals(StageEditStatus::Idle))
                    .and_then(any_with_component::<OnPick>())
                ),
            );
    }
}


#[derive(SystemParam, Debug)]
pub struct PickedItemsParam<'w, 's> {
    picked_items: Query<'w, 's, Entity, With<OnPick>>,
}

impl<'w, 's> PickedItemsParam<'w, 's> {
    pub fn remove_picked(&self, commands: &mut Commands) {
        for entity in self.picked_items.iter() {
            commands.entity(entity).remove::<OnPick>();
        }
    }
}


fn update(
    assets: Res<GimmickAssets>,
    page_index: Res<PageIndex>,
    mut commands: Commands,
    item: Query<&OnPick, With<OnPick>>,
    floors: Query<(&Transform, &SpriteInteraction), (With<SpriteInteraction>, With<Floor>)>,
) {
    for (transform, interaction, ) in floors.iter() {
        if interaction.is_clicked() {
            let OnPick(tag) = item.single();

            commands
                .spawn(gimmick_iem_sprite_bundle(front(transform.translation), tag.image(&assets)))
                .insert(Gimmick(*tag))
                .insert(PageIndex::new(page_index.0))
                .on_undo(|cmd, entity| {
                    cmd.entity(entity).despawn();
                });
            return;
        }
    }
}


#[cfg(test)]
mod tests {}