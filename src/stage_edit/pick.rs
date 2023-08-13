use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy_undo::prelude::{EntityCommandsOnUndoBuilderExt, EntityCommandsOnUndoExt};

use crate::assets::gimmick::GimmickAssets;
use crate::assets::stage_edit_assets::StageEditAssets;
use crate::button::{SpriteButton, SpriteInteraction};
use crate::extension::InteractionCondition;
use crate::gama_state::GameState;
use crate::page::page_index::PageIndex;
use crate::stage::playing::gimmick::{Floor, Gimmick, GimmickItem, GimmickItemSpawned};
use crate::stage_edit::{gimmick_iem_sprite_bundle, StageEditStatus};
use crate::stage_edit::idle::OnPick;
use crate::stage_edit::ui::item_area::{ItemArea, ItemPlusButton};
use crate::stage_edit::ui::new_gimmick_ui_image;

#[derive(Debug, Default, Copy, Clone, Hash, Eq, PartialEq)]
pub struct StageEditPickedPlugin;


impl Plugin for StageEditPickedPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update,
                (
                    spawn_gimmick_system,
                    add_item_system
                )
                    .run_if(in_state(GameState::StageEdit)
                        .and_then(resource_exists_and_equals(StageEditStatus::Idle))
                        .and_then(any_with_component::<OnPick>())
                    ),
            );
    }
}


#[derive(SystemParam, Debug)]
pub struct PickedItemsParam<'w, 's> {
    pub edit_assets: Res<'w, StageEditAssets>,
    picked_items: Query<'w, 's, Entity, With<OnPick>>,
}


impl<'w, 's> PickedItemsParam<'w, 's> {
    pub fn remove_picked(&self, commands: &mut Commands) {
        for entity in self.picked_items.iter() {
            commands.entity(entity).remove::<OnPick>();
        }
    }
}


fn spawn_gimmick_system(
    mut commands: Commands,
    assets: Res<GimmickAssets>,
    page_index: Res<PageIndex>,
    picked: Query<&OnPick, With<OnPick>>,
    floors: Query<(&Transform, &SpriteInteraction), (With<SpriteInteraction>, With<Floor>)>,
) {
    for (transform, interaction, ) in floors.iter() {
        if interaction.is_clicked() {
            let OnPick(tag) = picked.single();

            commands
                .spawn(gimmick_iem_sprite_bundle(transform.translation + Vec3::new(0., 0., 1.), tag.image(&assets)))
                .insert((
                    SpriteButton,
                    SpriteInteraction::None,
                    GimmickItemSpawned(*tag),
                    Gimmick,
                    *tag,
                    PageIndex::new(page_index.0)
                ))
                .on_undo(|cmd, entity| {
                    cmd.entity(entity).despawn();
                });
            return;
        }
    }
}


fn add_item_system(
    mut commands: Commands,
    page_index: Res<PageIndex>,
    mouse: Res<Input<MouseButton>>,
    assets: Res<GimmickAssets>,
    picked: Query<&OnPick, With<OnPick>>,
    item_area: Query<(Entity, &PageIndex), With<ItemArea>>,
    item_plus: Query<(&Interaction, &PageIndex), With<ItemPlusButton>>,
) {
    let Some((interaction, page_index)) = item_plus
        .iter().find(|(_, idx)| **idx == *page_index) else { return; };

    let Some((item_area, _)) = item_area
        .iter().find(|(_, idx)| **idx == *page_index) else { return; };

    if mouse.just_pressed(MouseButton::Left) && interaction.pressed() {
        let OnPick(tag) = picked.single();
        let mut item = commands.spawn_empty();
        item
            .insert(new_gimmick_ui_image(*tag, &assets))
            .insert(GimmickItem(*tag))
            .insert(*page_index);

        let item_entity = item.id();
        commands
            .entity(item_area)
            .insert_children(0, &[item_entity])
            .on_undo_builder()
            .add_entity(item_entity)
            .on_undo(|command, (item_area, item)| {
                command
                    .entity(item_area)
                    .remove_children(&[item]);
            });
    }
}


#[cfg(test)]
mod tests {}