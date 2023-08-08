use std::collections::HashMap;

use bevy::app::{App, Plugin, Update};
use bevy::math::{I64Vec2, Vec2};
use bevy::prelude::{Condition, in_state, IntoSystemConfigs, NextState, Query, ResMut, Transform, With};
use bevy_egui::{egui, EguiContexts};

use crate::gama_state::GameState;
use crate::loader::{StageLoadable, StageLoader};
use crate::loader::json::{Page, StageCell, StageJson};
use crate::page::page_index::PageIndex;
use crate::page::page_param::PageParams;
use crate::stage::playing::gimmick::Gimmick;
use crate::stage::playing::gimmick::tag::GimmickTag;
use crate::stage_edit::stage_name::StageName;
use crate::stage_edit::StageEditState;

#[derive(Default, Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct StageEditSavePlugin;


impl Plugin for StageEditSavePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, save_ui
                .run_if(in_state(GameState::StageEdit).and_then(in_state(StageEditState::Save))),
            );
    }
}


fn save_ui(
    mut state: ResMut<NextState<StageEditState>>,
    mut stage_name: ResMut<StageName>,
    page_params: PageParams,
    stage_cells: Query<(&Transform, &Gimmick, &PageIndex), (With<Transform>, With<Gimmick>, With<PageIndex>)>,
    mut context: EguiContexts,
) {
    egui::CentralPanel::default()
        .show(context.ctx_mut(), |ui| {
            ui.vertical_centered(|ui| {
                ui.text_edit_singleline(&mut stage_name.0);

                ui.vertical_centered(|ui| {
                    if ui.button("Cancel").clicked() {
                        state.set(StageEditState::Idle);
                    }

                    if ui.button("Save").clicked() {
                        save_stage(stage_name.0.clone(), page_params, &stage_cells);
                    }
                });
            });
        });
}


fn save_stage(
    stage_name: String,
    page_params: PageParams,
    stage_cells: &Query<(&Transform, &Gimmick, &PageIndex), (With<Transform>, With<Gimmick>, With<PageIndex>)>,
) {
    let pages = (0..page_params.page_count())
        .map(|page_index| create_page_asset(page_index, stage_cells))
        .collect::<Vec<Page>>();

    let json = StageJson {
        name: stage_name,
        pages,
    };
    StageLoader::new().save(&json).unwrap();
}


fn create_page_asset(
    page_index: usize,
    stage_cells: &Query<(&Transform, &Gimmick, &PageIndex), (With<Transform>, With<Gimmick>, With<PageIndex>)>,
) -> Page {
    let mut cells = Vec::new();

    for (pos, tags) in cells_in_page(page_index, stage_cells) {
        cells.push(StageCell::new(Vec2::new(pos.x as f32, pos.y as f32), tags));
    }

    Page {
        cells
    }
}


fn cells_in_page(
    page_index: usize,
    stage_cells: &Query<(&Transform, &Gimmick, &PageIndex), (With<Transform>, With<Gimmick>, With<PageIndex>)>,
) -> HashMap<I64Vec2, Vec<GimmickTag>> {
    let mut stage = HashMap::<I64Vec2, Vec<GimmickTag>>::new();

    stage_cells
        .iter()
        .filter(|(_, _, idx)| ***idx == page_index)
        .for_each(|(transform, gimmick, _)| {
            let key = transform.translation.truncate().as_i64vec2();
            if let std::collections::hash_map::Entry::Vacant(e) = stage.entry(key) {
                e.insert(vec![gimmick.0]);
            } else {
                stage
                    .get_mut(&key)
                    .unwrap()
                    .push(gimmick.0);
            }
        });

    stage
}