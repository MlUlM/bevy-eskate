use bevy::prelude::*;
use bevy::utils::default;

use crate::gama_state::GameState;
use crate::gimmick::{floor, Floor, GIMMICK_HEIGHT, GIMMICK_HEIGHT_PX, GIMMICK_WIDTH, GimmickItem, Stage};
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
    commands.spawn(Camera2dBundle::default());

    commands.spawn(NodeBundle {
        style: Style {
            height: Val::Percent(100.),
            width: Val::Percent(100.),
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },

        ..Default::default()
    })
        .with_children(|parent| children(parent, &asset));



    for x in 0..=24u8 {
        for y in 0..=12u8 {
            let x = f32::from(x) * 50. - 12. * 50.;
            let y = f32::from(y) * 50. - 3.5 * 50.;
            commands.spawn(ButtonBundle {
                style: Style{
                    height: GIMMICK_HEIGHT_PX,
                    width: GIMMICK_HEIGHT_PX,
                    ..default()
                },
                image: GimmickTag::Floor.load_to_ui_image(&asset),
                transform: Transform::from_xyz(x, y, 3.),
                ..default()
            })
                .insert(Floor)
            ;
        }
    }
}


#[inline]
fn children(parent: &mut ChildBuilder, asset: &AssetServer) {
    center(parent, asset);
    footer(parent);
}


fn center(parent: &mut ChildBuilder, asset: &AssetServer) {
    parent.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            height: Val::Percent(80.),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },

        ..default()
    });
}


fn stage(parent: &mut ChildBuilder, asset: &AssetServer) {
    parent.spawn(NodeBundle {
        style: Style {
            width: Val::Px(GIMMICK_WIDTH * 24.),
            height: Val::Px(GIMMICK_HEIGHT * 12.),
            flex_wrap: FlexWrap::Wrap,
            display: Display::None,
            ..default()
        },
        background_color: BackgroundColor(Color::NONE),
        ..default()
    })
        .insert(Stage)
        .with_children(|parent| {
            for _ in 0..24 {
                for _ in 0..12 {
                    spawn(parent, floor::texture(asset));
                }
            }
        });
}


fn spawn(
    parent: &mut ChildBuilder,
    texture: Handle<Image>,
) {
    parent.spawn(ButtonBundle {
        style: Style {
            height: Val::Px(GIMMICK_HEIGHT),
            width: Val::Px(GIMMICK_WIDTH),
            ..default()
        },
        image: UiImage::new(texture),
        ..default()
    })
        .insert(Floor);
}


fn footer(parent: &mut ChildBuilder) {
    parent.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            height: Val::Percent(20.),
            align_items: AlignItems::Center,
            column_gap: Val::Px(10.),
            ..default()
        },
        background_color: BackgroundColor(Color::BLACK),
        ..default()
    })
        .with_children(|parent| {
            parent.spawn(ButtonBundle {
                style: Style {
                    height: Val::Percent(80.),
                    aspect_ratio: Some(1.),
                    ..default()
                },
                background_color: BackgroundColor(Color::WHITE),
                ..default()
            })
                .insert(GimmickItem(GimmickTag::Rock));
        });
}
