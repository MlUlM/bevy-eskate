use bevy::prelude::*;
use bevy::utils::default;

use crate::button::{SpriteButton, SpriteInteraction};
use crate::gama_state::GameState;
use crate::gimmick::{Floor, Gimmick, GIMMICK_SIZE, GimmickItem, Stage};
use crate::gimmick::asset::GimmickAssets;
use crate::gimmick::tag::GimmickTag;
use crate::playing::PageIndex;
use crate::stage_edit::idle::{NextablePage, StageEditIdlePlugin};
use crate::stage_edit::page_count::StageEditPageCount;
use crate::stage_edit::pick::StageEditPickedPlugin;

#[derive(Default, Debug, Hash, Eq, PartialEq, States, Copy, Clone)]
pub enum StageEditState {
    #[default]
    Idle,

    PickItem,
}


mod idle;
mod pick;
pub mod page_count;
mod page_param;


#[derive(Default, Debug, PartialEq, Copy, Clone)]
pub struct StageEditPlugin;


impl Plugin for StageEditPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<StageEditState>()
            .add_systems(OnEnter(GameState::StageEdit), setup_stage_editor)
            .add_systems(Update, change_visible_gimmicks.run_if(in_state(GameState::StageEdit).and_then(resource_changed::<PageIndex>())))
            .add_plugins(StageEditIdlePlugin)
            .add_plugins(StageEditPickedPlugin);
    }
}


fn setup_stage_editor(
    page_count: Res<StageEditPageCount>,
    mut commands: Commands,
    assets: Res<GimmickAssets>,
) {
    commands.spawn(Camera2dBundle::default()).insert(Stage);
    commands.insert_resource(PageIndex::default());
    commands.spawn(NextablePage);

    ui(&mut commands, &assets);
    spawn_stage_gimmicks(&mut commands, &assets, page_count.0);
}


fn change_visible_gimmicks(
    page_index: Res<PageIndex>,
    mut gimmicks: Query<(&PageIndex, &mut Visibility), (
        With<PageIndex>,
        With<Gimmick>,
        With<Visibility>
    )>,
) {
    for (idx, mut visibility) in gimmicks.iter_mut() {
        if idx.0 == page_index.0 {
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}


fn spawn_stage_gimmicks(
    commands: &mut Commands,
    assets: &GimmickAssets,
    page_count: usize,
) {
    for page_index in 0..page_count {
        let page_index = PageIndex::new(page_index);
        for x in 0..=24u8 {
            for y in 0..=12u8 {
                let x = f32::from(x) * 50. - 12. * 50.;
                let y = f32::from(y) * 50. - 3.5 * 50.;

                commands
                    .spawn(gimmick_iem_sprite_bundle(Vec3::new(x, y, 0.), GimmickTag::Floor.image(assets)))
                    .insert((Floor, Gimmick(GimmickTag::Floor), SpriteButton, SpriteInteraction::None, page_index));
            }
        }
    }
}


fn ui(commands: &mut Commands, asset: &GimmickAssets) {
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
                GimmickTag::FallDown,
                GimmickTag::Goal
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
            height: Val::Percent(80.),
            aspect_ratio: Some(1.),
            margin: UiRect::left(Val::Px(20.)),
            ..default()
        },
        image: gimmick_tag.ui_image(asset),
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


#[cfg(test)]
mod tests {
    use bevy::prelude::*;

    use crate::gimmick::asset::GimmickAssets;
    use crate::playing::PageIndex;
    use crate::stage_edit::{change_visible_gimmicks, setup_stage_editor, StageEditPageCount, StageEditState};

    pub(crate) fn new_stage_edit_app(page_count: StageEditPageCount) -> App {
        let mut app = App::new();
        app.insert_resource(page_count);
        app.insert_resource(GimmickAssets::default());
        app.add_state::<StageEditState>();
        app.insert_resource(PageIndex::default());

        app
    }

    #[test]
    fn setup_stage_editor_page2() {
        let mut app = App::new();
        app.add_systems(Startup, setup_stage_editor);
        app.insert_resource(StageEditPageCount::new(2));
        app.insert_resource(GimmickAssets::default());

        app.update();

        let exists_page_0_gimmicks = app
            .world
            .query::<&PageIndex>()
            .iter(&app.world)
            .any(|page_index| page_index.0 == 0);
        assert!(exists_page_0_gimmicks);

        let exists_page_1_gimmicks = app
            .world
            .query::<&PageIndex>()
            .iter(&app.world)
            .any(|page_index| page_index.0 == 1);
        assert!(exists_page_1_gimmicks);
    }


    #[test]
    fn changed_invisible_page1_gimmicks() {
        let mut app = App::new();
        app.add_systems(Startup, setup_stage_editor);
        app.add_systems(Update, change_visible_gimmicks.run_if(
            resource_changed::<PageIndex>()
        ));
        app.insert_resource(StageEditPageCount::new(2));
        app.insert_resource(GimmickAssets::default());

        app.update();

        let all_visible_page0_gimmicks = app
            .world
            .query::<(&PageIndex, &Visibility)>()
            .iter(&app.world)
            .filter(|(page_index, _)| page_index.0 == 0)
            .all(|(_, visibility)| visibility == Visibility::Visible);
        assert!(all_visible_page0_gimmicks);

        let all_hidden_page1_gimmicks = app
            .world
            .query::<(&PageIndex, &Visibility)>()
            .iter(&app.world)
            .filter(|(page_index, _)| page_index.0 == 1)
            .all(|(_, visibility)| visibility == Visibility::Hidden);
        assert!(all_hidden_page1_gimmicks);
    }


    #[test]
    fn changed_visible_gimmicks_if_page_index_changed() {
        let mut app = App::new();
        app.add_systems(Startup, setup_stage_editor);
        app.add_systems(Update, change_visible_gimmicks.run_if(
            resource_changed::<PageIndex>()
        ));
        app.insert_resource(StageEditPageCount::new(2));
        app.insert_resource(GimmickAssets::default());

        app.update();

        *app
            .world
            .resource_mut::<PageIndex>()
            = PageIndex::new(1);

        app.update();

        let all_invisible_page0_gimmicks = app
            .world
            .query::<(&PageIndex, &Visibility)>()
            .iter(&app.world)
            .filter(|(page_index, _)| page_index.0 == 0)
            .all(|(_, visibility)| visibility == Visibility::Hidden);
        assert!(all_invisible_page0_gimmicks);

        let all_visible_page1_gimmicks = app
            .world
            .query::<(&PageIndex, &Visibility)>()
            .iter(&app.world)
            .filter(|(page_index, _)| page_index.0 == 1)
            .all(|(_, visibility)| visibility == Visibility::Visible);
        assert!(all_visible_page1_gimmicks);
    }
}