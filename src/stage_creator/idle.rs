use bevy::prelude::*;

use crate::gama_state::GameState;
use crate::gimmick::GimmickItem;
use crate::gimmick::tag::GimmickTag;
use crate::stage_creator::StageCreatorState;

#[derive(Debug,  Copy, Clone, Component, Eq, PartialEq)]
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
    mut state: ResMut<NextState<StageCreatorState>>,
    mut commands: Commands,
    items: Query<(Entity, &Interaction, &GimmickItem), (With<Button>, With<GimmickItem>)>,
) {
    for (entity, interaction, GimmickItem(tag)) in items.iter() {
        if interaction == &Interaction::Pressed {
            state.set(StageCreatorState::PickItem);
            commands.entity(entity).insert(OnPick(*tag));
            return;
        }
    }
}