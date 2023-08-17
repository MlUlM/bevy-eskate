use bevy::app::{App, Plugin, Update};
use bevy::prelude::{Commands, Component, Condition, Entity, in_state, IntoSystemConfigs, Query, Resource, resource_exists, With};

use crate::button::SpriteInteraction;
use crate::gama_state::GameState;
use crate::stage::playing::gimmick::GimmickItemSpawned;

#[derive(Component, Resource, Copy, Clone, Default, Eq, PartialEq, Hash)]
pub struct OnPickedEraser;

#[derive(Copy, Clone, Default, Eq, PartialEq, Hash)]
pub struct StageEditEraserPlugin;


impl Plugin for StageEditEraserPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update,
                erase_gimmick_system
                    .run_if(in_state(GameState::StageEdit)
                        .and_then(resource_exists::<OnPickedEraser>())
                    ),
            );
    }
}


fn erase_gimmick_system(
    mut commands: Commands,
    gimmicks: Query<(Entity, &SpriteInteraction), With<GimmickItemSpawned>>,
) {
    for (entity, interaction) in gimmicks.iter() {
        if interaction.just_pressed() {
            commands
                .entity(entity)
                .despawn();

            return;
        }
    }
}