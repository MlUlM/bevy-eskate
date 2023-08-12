use bevy::prelude::*;

use crate::{destroy_all, reset_game_cursor};
use crate::assets::gimmick::GimmickAssets;
use crate::assets::stage_edit_assets::StageEditAssets;
use crate::button::{SpriteButton, SpriteInteraction};
use crate::gama_state::GameState;
use crate::page::page_count::PageCount;
use crate::page::page_index::PageIndex;
use crate::stage::playing::gimmick::{Floor, Gimmick, GIMMICK_HEIGHT, GIMMICK_WIDTH};
use crate::stage::playing::gimmick::tag::GimmickTag;
use crate::stage_edit::idle::StageEditIdlePlugin;
use crate::stage_edit::pick::StageEditPickedPlugin;
use crate::stage_edit::save::StageEditSavePlugin;
use crate::stage_edit::stage_name::StageName;
use crate::stage_edit::ui::{gimmick_iem_sprite_bundle, spawn_ui};

#[derive(Default, Debug, Hash, Eq, PartialEq, Copy, Clone, Resource)]
pub enum StageEditStatus {
    #[default]
    Idle,

    SaveStage,

    AddingItem,
}


mod idle;
mod pick;
mod save;
mod stage_name;
pub mod ui;


#[derive(Default, Debug, PartialEq, Copy, Clone)]
pub struct StageEditPlugin;


impl Plugin for StageEditPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::StageEdit), setup)
            .add_systems(OnExit(GameState::StageEdit), (destroy_all, reset_game_cursor))
            .add_systems(Update, change_visible_gimmicks.run_if(in_state(GameState::StageEdit).and_then(resource_changed::<PageIndex>())))
            .add_plugins((
                StageEditIdlePlugin,
                StageEditPickedPlugin,
                StageEditSavePlugin
            ));
    }
}


fn setup(
    mut commands: Commands,
    page_count: Res<PageCount>,
    assets: Res<GimmickAssets>,
    edit_assets: Res<StageEditAssets>,
) {
    commands.insert_resource(StageEditStatus::default());
    commands.insert_resource(PageIndex::default());
    commands.insert_resource(StageName::default());

    spawn_ui(&mut commands, &assets, &edit_assets, *page_count);
    spawn_stage_gimmicks(&mut commands, &assets, page_count.0);
}


fn change_visible_gimmicks(
    page_index: Res<PageIndex>,
    mut gimmicks: Query<(&PageIndex, &mut Visibility), (
        With<PageIndex>,
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
                if x == 0 || y == 0 || x == 24 || y == 12 {
                    let tag = if (x == 0 || x == 24) && 0 < y { GimmickTag::WallSide } else { GimmickTag::Wall };
                    let x = f32::from(x) * GIMMICK_WIDTH - 12. * GIMMICK_WIDTH;
                    let y = f32::from(y) * GIMMICK_HEIGHT - 3.5 * GIMMICK_HEIGHT;
                    commands
                        .spawn(gimmick_iem_sprite_bundle(Vec3::new(x, y, 0.), tag.image(assets)))
                        .insert((Gimmick(tag), SpriteButton, SpriteInteraction::None, page_index));
                } else {
                    let x = f32::from(x) * GIMMICK_WIDTH - 12. * GIMMICK_WIDTH;
                    let y = f32::from(y) * GIMMICK_HEIGHT - 3.5 * GIMMICK_HEIGHT;
                    commands
                        .spawn(gimmick_iem_sprite_bundle(Vec3::new(x, y, 0.), GimmickTag::Floor.image(assets)))
                        .insert((Floor, Gimmick(GimmickTag::Floor), SpriteButton, SpriteInteraction::None, page_index));
                };
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use bevy::prelude::*;

    use crate::assets::gimmick::GimmickAssets;
    use crate::page::page_index::PageIndex;
    use crate::stage_edit::{change_visible_gimmicks, PageCount, setup, StageEditStatus};

    pub(crate) fn new_stage_edit_app(page_count: PageCount) -> App {
        let mut app = App::new();
        app.init_resource::<StageEditStatus>();
        app.init_resource::<PageIndex>();
        app.init_resource::<GimmickAssets>();
        app.insert_resource(page_count);

        app
    }


    #[test]
    fn setup_stage_editor_page2() {
        let mut app = App::new();
        app.add_systems(Startup, setup);
        app.insert_resource(PageCount::new(2));
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
        app.add_systems(Startup, setup);
        app.add_systems(Update, change_visible_gimmicks.run_if(
            resource_changed::<PageIndex>()
        ));
        app.insert_resource(PageCount::new(2));
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
        app.add_systems(Startup, setup);
        app.add_systems(Update, change_visible_gimmicks.run_if(
            resource_changed::<PageIndex>()
        ));
        app.insert_resource(PageCount::new(2));
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