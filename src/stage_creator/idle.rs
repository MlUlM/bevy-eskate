use std::collections::HashMap;

use bevy::math::I64Vec2;
use bevy::prelude::*;

use crate::gama_state::GameState;
use crate::gimmick::{Gimmick, GimmickItem};
use crate::gimmick::tag::GimmickTag;
use crate::loader::{StageLoadable, StageLoader};
use crate::loader::json::{Page, StageCell, StageJson};
use crate::stage_creator::StageCreatorState;

#[derive(Debug, Copy, Clone, Component, Eq, PartialEq)]
pub struct OnPick(pub GimmickTag);


#[derive(Debug, Default, Copy, Clone, Hash, Eq, PartialEq)]
pub struct StageCreatorIdlePlugin;


impl Plugin for StageCreatorIdlePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, update.run_if(in_state(GameState::StageCreator).and_then(in_state(StageCreatorState::Idle))));
    }
}


fn update(
    key: Res<Input<KeyCode>>,
    mut state: ResMut<NextState<StageCreatorState>>,
    mut commands: Commands,
    items: Query<(Entity, &Interaction, &GimmickItem), (With<Button>, With<GimmickItem>)>,
    cells: Query<(&Transform, &Gimmick), (With<Transform>, With<Gimmick>)>,
) {
    if key.just_pressed(KeyCode::Return) {
        println!("save");
        stage_save(cells);
        return;
    }

    for (entity, interaction, GimmickItem(tag)) in items.iter() {
        if interaction == &Interaction::Pressed {
            state.set(StageCreatorState::PickItem);
            commands
                .entity(entity)
                .insert(OnPick(*tag));

            return;
        }
    }
}


fn stage_save(
    cells: Query<(&Transform, &Gimmick), (With<Transform>, With<Gimmick>)>
) {
    let mut stage_cells = Vec::new();
    for (pos, tags) in stage(cells) {
        stage_cells.push(StageCell::new(Vec2::new(pos.x as f32, pos.y as f32), tags));
    }

    let json = StageJson {
        name: "stage1".to_string(),
        pages: vec![{
            Page {
                cells: stage_cells
            }
        }],
    };
    StageLoader::new().save(&json).unwrap();
}


fn stage(
    cells: Query<(&Transform, &Gimmick), (With<Transform>, With<Gimmick>)>
) -> HashMap<I64Vec2, Vec<GimmickTag>> {
    let mut stage = HashMap::<I64Vec2, Vec<GimmickTag>>::new();
    cells.for_each(|(transform, gimmick)| {
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
    #[test]
    fn create() {}
}