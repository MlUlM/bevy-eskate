use bevy::ecs::system::SystemParam;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy_undo2::prelude::{UndoCallbackEvent, UndoScheduler};
use itertools::Itertools;

use crate::assets::gimmick::GimmickAssets;
use crate::assets::stage_edit_assets::StageEditAssets;
use crate::button::{SpriteButton, SpriteInteraction};
use crate::gama_state::GameState;
use crate::page::page_index::PageIndex;
use crate::stage::playing::gimmick::{Floor, Gimmick, GIMMICK_HEIGHT, GimmickItem, GimmickItemSpawned};
use crate::stage_edit::{gimmick_sprite_bundle, StageEditStatus};
use crate::stage_edit::idle::OnPick;
use crate::stage_edit::page::item_area::ItemPlusButton;

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
    mut scheduler: UndoScheduler<UndoCallbackEvent>,
    assets: Res<GimmickAssets>,
    gimmicks: Query<(&Transform, &PageIndex), With<GimmickItemSpawned>>,
    picked: Query<&OnPick, With<OnPick>>,
    floors: Query<(&Transform, &SpriteInteraction, &PageIndex, &Parent), (With<SpriteInteraction>, With<Floor>)>,
) {
    for (transform, interaction, page_index, parent) in floors.iter() {
        if interaction.just_pressed() {
            let OnPick(tag) = picked.single();
            let z = gimmicks
                .iter()
                .filter(|(_, idx)| **idx == *page_index)
                .filter(|(t, _)| t.translation.xy().abs_diff_eq(transform.translation.xy(), 0.1))
                .map(|(transform, _)| transform.translation.z)
                .sorted_by(|z1, z2| {
                    z1.partial_cmp(z2).unwrap()
                })
                .last()
                .unwrap_or_default();

            let gimmick = commands
                .spawn(gimmick_sprite_bundle(transform.translation + Vec3::Z + Vec3::Z * z, tag.image(&assets)))
                .insert((
                    SpriteButton,
                    SpriteInteraction::None,
                    GimmickItemSpawned(*tag),
                    Gimmick,
                    *tag,
                    *page_index
                ))
                .id();

            let parent = parent.get();
            commands.entity(parent).add_child(gimmick);
            scheduler.register(UndoCallbackEvent::new(move |cmd| {
                cmd.entity(parent).remove_children(&[gimmick]);
            }));
            return;
        }
    }
}


fn add_item_system(
    mut commands: Commands,
    mut scheduler: UndoScheduler<UndoCallbackEvent>,
    assets: Res<GimmickAssets>,
    picked: Query<&OnPick, With<OnPick>>,
    item_area: Query<(Entity, &SpriteInteraction, &PageIndex, Option<&Children>), With<ItemPlusButton>>,
    items: Query<&Transform, With<GimmickItem>>,
) {
    let Some((item_area_entity, _, page_index, children)) = item_area
        .iter()
        .find(|(_, interaction, _, _)| interaction.just_pressed()) else { return; };

    let OnPick(tag) = picked.single();
    let pos = children
        .and_then(|children| children
            .iter()
            .filter_map(|c| items.get(*c).ok())
            .map(|ct| ct.translation + Vec3::new(0., -GIMMICK_HEIGHT - 8., 0.))
            .sorted_by(|p1, p2| p2.y.partial_cmp(&p1.y).unwrap())
            .last()
        )
        .unwrap_or(Vec3::NEG_Y * GIMMICK_HEIGHT);

    commands
        .entity(item_area_entity)
        .with_children(|parent| {
            let item = parent.spawn(gimmick_sprite_bundle(pos + Vec3::Z, tag.image(&assets)))
                .insert(GimmickItem(*tag))
                .insert(*page_index)
                .id();

            scheduler.register(UndoCallbackEvent::new(move |cmd| {
                cmd.entity(item).despawn();
            }));
        });
}


#[cfg(test)]
mod tests {}