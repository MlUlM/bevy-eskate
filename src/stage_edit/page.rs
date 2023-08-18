use bevy::hierarchy::{BuildChildren, ChildBuilder};
use bevy::math::Vec3;
use bevy::prelude::{Commands, Component, default, Transform};
use bevy::sprite::{Anchor, Sprite, SpriteBundle};
use itertools::Itertools;

use crate::assets::gimmick::GimmickAssets;
use crate::loader::json::PageJson;
use crate::page::page_index::PageIndex;
use crate::stage_edit::page::item_area::spawn_item_area;

pub mod item_area;

#[derive(Component, Copy, Clone, Eq, PartialEq, Debug, Default)]
pub struct Page;


#[derive(Component, Copy, Clone, Eq, PartialEq, Debug, Default)]
pub struct Field;


pub fn spawn_page(
    commands: &mut Commands,
    page: &PageJson,
    page_index: PageIndex,
    gimmick_assets: &GimmickAssets,
) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                anchor: Anchor::TopLeft,
                ..default()
            },
            transform: Transform::from_xyz(page.x, page.y, 0.),
            ..default()
        })
        .insert((Page, page_index))
        .with_children(|parent| {
            spawn_item_area(parent, gimmick_assets,&page.item_area, page_index);
            parent
                .spawn(SpriteBundle {
                    transform: Transform::from_xyz(page.item_area.width, 0., 0.),
                    ..default()
                })
                .insert((Field, page_index))
                .with_children(|panret| {
                    spawn_page_gimmicks(panret, page, page_index, gimmick_assets);
                });
        });
}


fn spawn_page_gimmicks(
    parent: &mut ChildBuilder,
    page: &PageJson,
    page_index: PageIndex,
    assets: &GimmickAssets,
) {
    for cell in page.cells.iter() {
        for (index, tag) in cell.tags.iter().sorted().enumerate() {
            tag.spawn_with_parent(parent, assets, Vec3::new(cell.x, cell.y, index as f32), page_index);
        }
    }
}

