use bevy::core::Name;
use bevy::hierarchy::BuildChildren;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{ChildBuilder};
use bevy::sprite::{Anchor, Sprite, SpriteBundle};
use bevy::utils::default;
use bevy_trait_query::imports::Component;
use crate::assets::gimmick::GimmickAssets;

use crate::button::{SpriteButton, SpriteButtonBundle, SpriteInteraction};
use crate::loader::json::ItemAreaJson;
use crate::page::page_index::PageIndex;
use crate::stage::playing::gimmick::GimmickItem;
use crate::stage_edit::ui::gimmick_sprite_bundle;


#[derive(Component, Eq, PartialEq, Copy, Clone)]
pub struct ItemArea;

#[derive(Component, Debug, Default, PartialEq, Eq, Copy, Clone)]
pub struct ItemPlusButton;


pub fn spawn_item_area(
    commands: &mut ChildBuilder,
    gimmick_assets: &GimmickAssets,
    item_area_json: &ItemAreaJson,
    page_index: PageIndex,
) {
    commands.spawn(SpriteButtonBundle::new(SpriteBundle {
        sprite: Sprite {
            // color: Color::from([0.6, 0.1, 0.1, 0.7]),
            custom_size: Some(Vec2::new(item_area_json.width, item_area_json.height)),
            anchor: Anchor::TopCenter,
            ..default()
        },
        ..default()
    }))
        .insert((Name::new("ItemArea"), ItemArea, ItemPlusButton, page_index))
        .with_children(|parent| {
            for item in item_area_json.tags.iter() {
                parent
                    .spawn(gimmick_sprite_bundle(Vec3::new(item.x, item.y, 0.), item.tag.image(gimmick_assets)))
                    .insert(SpriteInteraction::None)
                    .insert(SpriteButton)
                    .insert(GimmickItem(item.tag))
                    .insert(page_index);
            }
        });
}


