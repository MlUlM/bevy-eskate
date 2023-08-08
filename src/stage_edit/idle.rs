use std::collections::HashMap;

use bevy::math::I64Vec2;
use bevy::prelude::*;

use crate::gama_state::GameState;

use crate::loader::{StageLoadable, StageLoader};
use crate::loader::json::{Page, StageCell, StageJson};
use crate::playing::gimmick::{Gimmick, GimmickItem};
use crate::playing::gimmick::tag::GimmickTag;
use crate::playing::PageIndex;
use crate::stage_edit::page_param::PageParams;
use crate::stage_edit::StageEditState;

#[derive(Debug, Copy, Clone, Component, Eq, PartialEq)]
pub struct OnPick(pub GimmickTag);


#[derive(Debug, Default, Copy, Clone, Hash, Eq, PartialEq)]
pub struct StageEditIdlePlugin;


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum InputStatus {
    None,
    PickedItem(Entity, GimmickTag),
    SaveFile,
    NextPage,
    PreviousPage,
}


impl Plugin for StageEditIdlePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update,
                         click_pick_item
                             .pipe(input_handle)
                             .run_if(in_state(GameState::StageEdit).and_then(in_state(StageEditState::Idle))),
            );
    }
}


fn input_handle(
    In(input_status): In<InputStatus>,
    mut state: ResMut<NextState<StageEditState>>,
    mut commands: Commands,
    mut page_params: PageParams,
    stage_cells: Query<(&Transform, &Gimmick, &PageIndex), (With<Transform>, With<Gimmick>, With<PageIndex>)>,
) {
    match input_status {
        InputStatus::PickedItem(entity, gimmick_tag) => {
            state.set(StageEditState::PickItem);
            commands
                .entity(entity)
                .insert(OnPick(gimmick_tag));
        }
        InputStatus::SaveFile => {
            save_stage(page_params, &stage_cells);
        }
        InputStatus::NextPage => {
            page_params.next_page();
        }
        InputStatus::PreviousPage => {
            page_params.previous_page();
        }
        _ => {}
    }
}


fn click_pick_item(
    key: Res<Input<KeyCode>>,
    items: Query<(Entity, &Interaction, &GimmickItem), (With<Button>, With<GimmickItem>)>,
) -> InputStatus {
    if key.just_pressed(KeyCode::Return) {
        return InputStatus::SaveFile;
    }

    if key.just_pressed(KeyCode::Left) {
        return InputStatus::NextPage;
    }

    if key.just_pressed(KeyCode::Right) {
        return InputStatus::PreviousPage;
    }

    for (entity, interaction, GimmickItem(tag)) in items.iter() {
        if interaction == &Interaction::Pressed {
            return InputStatus::PickedItem(entity, *tag);
        }
    }

    InputStatus::None
}


fn save_stage(
    page_params: PageParams,
    stage_cells: &Query<(&Transform, &Gimmick, &PageIndex), (With<Transform>, With<Gimmick>, With<PageIndex>)>,
) {
    let pages = (0..page_params.page_count())
        .map(|page_index| create_page_asset(page_index, stage_cells))
        .collect::<Vec<Page>>();

    let json = StageJson {
        name: "stage1".to_string(),
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


#[cfg(test)]
mod tests {
    use bevy::app::Update;
    use bevy::prelude::IntoSystem;

    use crate::playing::PageIndex;
    use crate::stage_edit::idle::{input_handle, InputStatus};
    use crate::stage_edit::PageCount;
    use crate::stage_edit::tests::new_stage_edit_app;

    fn update_next_page() -> InputStatus {
        InputStatus::NextPage
    }

    #[test]
    fn unchanged_next_page_if_last_page() {
        let mut app = new_stage_edit_app(PageCount::new(2));
        app.add_systems(Update, update_next_page.pipe(input_handle));
        *app
            .world
            .resource_mut::<PageIndex>()
            = PageIndex::new(2);

        app.update();

        let page_index = app
            .world
            .resource::<PageIndex>();

        assert_eq!(*page_index, PageIndex(2));
    }


    #[test]
    fn increment_page_index_if_exists_nextable_page() {
        let mut app = new_stage_edit_app(PageCount::new(2));
        app.add_systems(Update, update_next_page.pipe(input_handle));

        app.update();

        let page_index = app
            .world
            .resource::<PageIndex>();
        assert_eq!(*page_index, PageIndex(1));
    }
}