use bevy::core::Name;
use bevy::hierarchy::BuildChildren;
use bevy::math::Vec2;
use bevy::prelude::{ChildBuilder, Color};
use bevy::sprite::{Anchor, Sprite, SpriteBundle};
use bevy::utils::default;
use bevy_trait_query::imports::Component;

use crate::button::SpriteButtonBundle;
use crate::loader::json::ItemAreaJson;
use crate::page::page_index::PageIndex;
use crate::stage_edit::ui::item_area::ItemPlusButton;

#[derive(Component, Eq, PartialEq, Copy, Clone)]
pub struct ItemArea;


pub fn spawn_item_area2(
    commands: &mut ChildBuilder,
    item_area_json: &ItemAreaJson,
    page_index: PageIndex,
) {
    commands.spawn(SpriteButtonBundle::new(SpriteBundle {
        sprite: Sprite {
            color: Color::from([0.6, 0.1, 0.1, 0.7]),
            custom_size: Some(Vec2::new(item_area_json.width, item_area_json.height)),
            anchor: Anchor::TopCenter,
            ..default()
        },
        ..default()
    }))
        .insert((Name::new("ItemArea"), ItemArea, ItemPlusButton, page_index))
        .with_children(|parent| {
            parent.spawn(SpriteBundle::default());
        });
}


// fn spawn_item_plus(parent: &mut ChildBuilder, assets: &StageEditAssets, page_index: PageIndex) {
//     parent.spawn(SpriteButtonBundle::new(SpriteBundle {
//         sprite: Sprite {
//             custom_size: Some(Vec2::new(32., 32.)),
//             ..default()
//         },
//         texture: assets.item_plus.clone(),
//         ..default()
//     }))
//         .insert((Name::new("ItemPlus")));
// }