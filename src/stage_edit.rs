use bevy::prelude::*;

use crate::{destroy_all, reset_game_cursor};
use crate::assets::gimmick::GimmickAssets;
use crate::assets::stage_edit_assets::StageEditAssets;
use crate::gama_state::GameState;
use crate::loader::json::StageJson;
use crate::page::page_count::PageCount;
use crate::page::page_index::PageIndex;
use crate::stage_edit::eraser::StageEditEraserPlugin;
use crate::stage_edit::idle::StageEditIdlePlugin;
use crate::stage_edit::page::spawn_page;
use crate::stage_edit::pick::StageEditPickedPlugin;
use crate::stage_edit::save::StageEditSavePlugin;
use crate::stage_edit::stage_name::StageName;
use crate::stage_edit::ui::{gimmick_sprite_bundle, spawn_ui};

#[derive(Default, Debug, Hash, Eq, PartialEq, Copy, Clone, Resource)]
pub enum StageEditStatus {
    #[default]
    Idle,

    SaveStage,

}


mod idle;
mod pick;
mod save;
pub mod stage_name;
pub mod ui;
mod eraser;
pub mod page;


#[derive(Default, Debug, PartialEq, Copy, Clone)]
pub struct StageEditPlugin;


impl Plugin for StageEditPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::StageEdit), setup)
            .add_systems(OnExit(GameState::StageEdit), (destroy_all, reset_game_cursor))
            // .add_systems(Update, change_visible_gimmicks.run_if(in_state(GameState::StageEdit).and_then(resource_changed::<PageIndex>())))
            .add_plugins((
                StageEditIdlePlugin,
                StageEditPickedPlugin,
                StageEditSavePlugin,
                StageEditEraserPlugin
            ));
    }
}


fn setup(
    mut commands: Commands,
    stage: Res<StageJson>,
    assets: Res<GimmickAssets>,
    edit_assets: Res<StageEditAssets>,
) {
    commands.insert_resource(StageEditStatus::default());
    commands.insert_resource(PageIndex::default());
    commands.insert_resource(StageName::default());
    commands.insert_resource(PageCount::new(stage.pages.len()));

    for (page_index, page) in stage.pages.iter().enumerate() {
        spawn_page(&mut commands, page, PageIndex::new(page_index), &assets);
    }
    spawn_ui(&mut commands, &assets, &edit_assets);
}


#[cfg(test)]
mod tests {
    use bevy::prelude::*;

    use crate::assets::gimmick::GimmickAssets;
    use crate::assets::stage_edit_assets::StageEditAssets;
    use crate::gama_state::GameState;
    use crate::loader::json::StageJson;
    use crate::page::page_index::PageIndex;
    use crate::stage_edit::{change_visible_gimmicks, PageCount, setup, StageEditStatus};
    use crate::stage_edit::idle::UserInputEvent;

    pub(crate) fn new_stage_edit_app(page_count: PageCount) -> App {
        let mut app = App::new();
        app.init_resource::<StageEditStatus>();
        app.init_resource::<PageIndex>();
        app.init_resource::<GimmickAssets>();
        app.insert_resource(StageJson::empty_stage(page_count, 15, 25, Vec2::ZERO));
        app.add_event::<UserInputEvent>();
        app.add_state::<GameState>();
        app.insert_resource(StageEditAssets::default());

        app
    }


    #[test]
    fn setup_stage_editor_page2() {
        let mut app = new_stage_edit_app(PageCount::new(2));
        app.add_systems(Startup, setup);
        app.update();

        let exists_page_0_gimmicks = app
            .world
            .query::<&PageIndex>()
            .iter(&app.world)
            .any(|page_index| {
                page_index.0 == 0
            });
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
        let mut app = new_stage_edit_app(PageCount::new(2));
        app.add_systems(Startup, setup);
        app.add_systems(Update, change_visible_gimmicks.run_if(
            resource_changed::<PageIndex>()
        ));


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
        let mut app = new_stage_edit_app(PageCount::new(2));
        app.add_systems(Startup, setup);
        app.add_systems(Update, change_visible_gimmicks.run_if(
            resource_changed::<PageIndex>()
        ));

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