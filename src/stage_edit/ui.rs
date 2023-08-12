use bevy::asset::Handle;
use bevy::hierarchy::{BuildChildren, ChildBuilder};
use bevy::math::Vec3;
use bevy::prelude::{AlignItems, BackgroundColor, ButtonBundle, Color, Commands, default, FlexDirection, Image, ImageBundle, JustifyContent, NodeBundle, PositionType, Sprite, SpriteBundle, Style, Transform, UiRect, Val};

use crate::assets::gimmick::GimmickAssets;
use crate::assets::stage_edit_assets::StageEditAssets;
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
    page_count: PageCount
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
                    for i in 0..*page_count{
                        spawn_item_area(parent, edit_assets, PageIndex::new(i));
                    }
                });

            footer(parent, asset);
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

fn footer(parent: &mut ChildBuilder, asset: &GimmickAssets) {
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
                GimmickTag::IceBox
            ]);
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




pub(crate) fn new_gimmick_ui_image(
    gimmick_tag: GimmickTag,
    asset: &GimmickAssets,
) -> ImageBundle {
    ImageBundle{
        style: Style {
            height: Val::Px(GIMMICK_HEIGHT),
            width: Val::Px(GIMMICK_WIDTH),
            ..default()
        },
        image: gimmick_tag.ui_image(asset),
        ..default()
    }
}

#[inline]
pub(crate) fn front(pos: Vec3) -> Vec3 {
    Vec3::new(pos.x, pos.y, 1.)
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