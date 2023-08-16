use bevy::asset::Handle;
use bevy::core::Name;
use bevy::hierarchy::{BuildChildren, ChildBuilder};
use bevy::math::Vec3;
use bevy::prelude::{AlignItems, BackgroundColor, ButtonBundle, Color, Commands, default, FlexDirection, Image, ImageBundle, JustifyContent, NodeBundle, Sprite, SpriteBundle, Style, Transform, UiImage, UiRect, Val};
use bevy_trait_query::imports::Component;

use crate::assets::gimmick::GimmickAssets;
use crate::assets::stage_edit_assets::StageEditAssets;
use crate::loader::json::StageJson;
use crate::page::page_count::PageCount;
use crate::page::page_index::PageIndex;
use crate::stage::playing::gimmick::{GIMMICK_HEIGHT, GIMMICK_SIZE, GIMMICK_WIDTH, GimmickItem};
use crate::stage::playing::gimmick::tag::GimmickTag;
use crate::stage_edit::ui::item_area::spawn_item_area;

pub mod item_area;


pub fn spawn_ui(
    commands: &mut Commands,
    asset: &GimmickAssets,
    edit_assets: &StageEditAssets,
    page_count: PageCount,
    stage: &StageJson,
) {
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        ..default()
    })
        .with_children(|parent| {
            parent.spawn(NodeBundle {
                style: Style {
                    height: Val::Percent(100.),
                    width: Val::Px(100.),
                    justify_content: JustifyContent::Center,
                    padding: UiRect::all(Val::Px(8.)),
                    ..default()
                },
                background_color: BackgroundColor::from(Color::Rgba { red: 122. / 255., green: 111. / 255., blue: 102. / 255., alpha: 1. }),
                ..default()
            })
                .with_children(|parent| {
                    for i in 0..*page_count {
                        spawn_item_area(parent, asset, edit_assets, PageIndex::new(i), &stage.pages[i].items);
                    }
                });

            footer(parent, asset, edit_assets);
        });
}


macro_rules! spawn_footer_items {
    ($parent: expr, $asset: expr, items => [
        $($tag: expr),*
    ]) => {
        $(
        spawn_footer_gimmick_item($parent, $asset, $tag);
        )*
    };
}

fn footer(parent: &mut ChildBuilder, asset: &GimmickAssets, edit_assets: &StageEditAssets) {
    parent.spawn(NodeBundle {
        style: Style {
            height: Val::Percent(10.),
            width: Val::Percent(100.),
            align_items: AlignItems::Center,
            column_gap: Val::Px(10.),
            ..default()
        },
        background_color: BackgroundColor(Color::BLACK),
        ..default()
    })
        .with_children(|parent| {
            spawn_footer_items!(parent, asset, items => [
                GimmickTag::Player,
                GimmickTag::Rock,
                GimmickTag::NextPage,
                GimmickTag::Goal,
                GimmickTag::Stop,
                GimmickTag::IceBox,
                GimmickTag::Turn,
                GimmickTag::Key,
                GimmickTag::Lock
            ]);

            spawn_eraser(parent, edit_assets);
        });
}


fn spawn_footer_gimmick_item(
    parent: &mut ChildBuilder,
    asset: &GimmickAssets,
    gimmick_tag: GimmickTag,
) {
    parent.spawn(ButtonBundle {
        style: Style {
            height: Val::Px(GIMMICK_WIDTH),
            aspect_ratio: Some(1.),
            margin: UiRect::left(Val::Px(20.)),
            ..default()
        },
        image: gimmick_tag.ui_image(asset),
        ..default()
    })
        .insert(GimmickItem(gimmick_tag));
}


#[derive(Component, Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct GimmickEraser;


fn spawn_eraser(parent: &mut ChildBuilder, assets: &StageEditAssets) {
    parent.spawn(ButtonBundle {
        style: Style {
            width: Val::Px(GIMMICK_WIDTH),
            height: Val::Px(GIMMICK_HEIGHT),
            ..default()
        },
        image: UiImage::new(assets.eraser.clone()),
        ..default()
    })
        .insert((Name::new("Eraser"), GimmickEraser));
}


pub(crate) fn new_gimmick_ui_image(
    gimmick_tag: GimmickTag,
    asset: &GimmickAssets,
) -> ImageBundle {
    ImageBundle {
        style: Style {
            height: Val::Px(GIMMICK_HEIGHT),
            width: Val::Px(GIMMICK_WIDTH),
            ..default()
        },
        image: gimmick_tag.ui_image(asset),
        ..default()
    }
}


pub(crate) fn gimmick_iem_sprite_bundle(pos: Vec3, texture: Handle<Image>) -> SpriteBundle {
    SpriteBundle {
        transform: Transform::from_xyz(pos.x, pos.y, pos.z),
        texture,
        sprite: Sprite {
            custom_size: Some(GIMMICK_SIZE),
            ..default()
        },
        ..default()
    }
}