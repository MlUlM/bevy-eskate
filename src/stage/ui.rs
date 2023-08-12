use bevy::core::Name;
use bevy::hierarchy::BuildChildren;
use bevy::prelude::{ChildBuilder, Color, Commands, NodeBundle, Style};
use bevy::ui::{BackgroundColor, FlexDirection, PositionType, Val};
use bevy::utils::default;

use crate::assets::gimmick::GimmickAssets;
use crate::page::page_index::PageIndex;
use crate::stage::playing::gimmick::tag::GimmickTag;
use crate::stage_edit::ui::new_gimmick_ui_image;

pub fn spawn_item_area(
    commands: &mut Commands,
    gimmick_asset: &GimmickAssets,
    stage_items: Vec<GimmickTag>,
    page_index: PageIndex,
) {
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Px(100.),
            height: Val::Percent(100.),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(10.),
            position_type: PositionType::Absolute,
            ..default()
        },
        background_color: BackgroundColor::from(Color::BEIGE),
        ..default()
    })
        .insert(Name::new(format!("ItemArea {:?}", page_index)))
        .insert(page_index)
        .with_children(|parent| {
            spawn_items(parent, gimmick_asset, stage_items, page_index);
        });
}


fn spawn_items(
    parent: &mut ChildBuilder,
    gimmick_asset: &GimmickAssets,
    stage_items: Vec<GimmickTag>,
    page_index: PageIndex,
) {
    stage_items
        .iter()
        .for_each(|item_tag| {
            parent
                .spawn(new_gimmick_ui_image(*item_tag, gimmick_asset))
                .insert((Name::new(format!("Item {:?}", item_tag)), page_index));
        });
}