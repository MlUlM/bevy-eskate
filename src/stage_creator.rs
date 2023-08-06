use bevy::prelude::*;
use bevy::utils::default;

use crate::button::{SpriteButton, SpriteInteraction};
use crate::gama_state::GameState;
use crate::gimmick::{Floor, Gimmick, GIMMICK_SIZE, GimmickItem, Stage};
use crate::gimmick::tag::GimmickTag;
use crate::stage_creator::idle::StageCreatorIdlePlugin;
use crate::stage_creator::pick::StageCreatorPickedPlugin;

#[derive(Default, Debug, Hash, Eq, PartialEq, States, Copy, Clone)]
pub enum StageCreatorState {
    #[default]
    Idle,

    PickItem,
}


mod idle;
mod pick;

#[derive(Default, Debug, PartialEq, Copy, Clone)]
pub struct StageCreatorPlugin;


impl Plugin for StageCreatorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<StageCreatorState>()
            .add_systems(OnEnter(GameState::StageCreator), setup)
            .add_plugins(StageCreatorIdlePlugin)
            .add_plugins(StageCreatorPickedPlugin);
    }
}


fn setup(
    mut commands: Commands,
    asset: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default()).insert(Stage);
    ui(&mut commands, &asset);

    for x in 0..=24u8 {
        for y in 0..=12u8 {
            let x = f32::from(x) * 50. - 12. * 50.;
            let y = f32::from(y) * 50. - 3.5 * 50.;

            commands
                .spawn(gimmick_iem_sprite_bundle(Vec3::new(x, y, 0.), GimmickTag::Floor.load(&asset)))
                .insert((Floor, Gimmick(GimmickTag::Floor), SpriteButton, SpriteInteraction::None));
        }
    }
}


fn ui(commands: &mut Commands, asset: &AssetServer) {
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
                    height: Val::Percent(90.),
                    width: Val::Percent(100.),
                    ..default()
                },
                ..default()
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

fn footer(parent: &mut ChildBuilder, asset: &AssetServer) {
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
                GimmickTag::FallDown,
                GimmickTag::Goal
            ]);
        });
}


fn spawn_footer_gimmick_item(
    parent: &mut ChildBuilder,
    asset: &AssetServer,
    gimmick_tag: GimmickTag,
) {
    parent.spawn(ButtonBundle {
        style: Style {
            height: Val::Percent(80.),
            aspect_ratio: Some(1.),
            margin: UiRect::left(Val::Px(20.)),
            ..default()
        },
        image: gimmick_tag.load_to_ui_image(asset),
        ..default()
    })
        .insert(GimmickItem(gimmick_tag));
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