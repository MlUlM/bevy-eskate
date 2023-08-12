use bevy::app::{App, Update};
use bevy::prelude::{Commands, Condition, in_state, IntoSystemConfigs, Plugin, Query, Res, resource_exists_and_equals, Transform, Vec3, With};
use bevy_trait_query::imports::{Component, Entity};
use bevy_undo::prelude::EntityCommandsOnUndoBuilderExt;

use crate::assets::gimmick::GimmickAssets;
use crate::button::SpriteInteraction;
use crate::gama_state::GameState;
use crate::page::page_index::PageIndex;
use crate::stage::playing::gimmick::{Floor, GimmickItem, GimmickItemSpawned};
use crate::stage::status::StageStatus;

#[derive(Component, Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct OnPickedItem;


#[derive(Debug, Default, Copy, Clone, Hash, Eq, PartialEq)]
pub struct PlayingPickedItemPlugin;


impl Plugin for PlayingPickedItemPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update,
                spawn_item_system
                    .run_if(in_state(GameState::Stage)
                        .and_then(resource_exists_and_equals(StageStatus::playing_picked_item()))
                    ),
            );
    }
}


fn spawn_item_system(
    mut commands: Commands,
    assets: Res<GimmickAssets>,
    page_index: Res<PageIndex>,
    picked_item: Query<(Entity, &GimmickItem), With<OnPickedItem>>,
    floors: Query<(&SpriteInteraction, &Transform), With<Floor>>,
) {
    let Some((item_entity, GimmickItem(tag))) = picked_item.iter().next() else { return; };

    for (interaction, transform) in floors.iter() {
        if interaction.is_clicked() {
            tag
                .spawn(&mut commands, &assets, transform.translation + Vec3::new(0., 0., 1.), *page_index)
                .insert(GimmickItemSpawned(*tag))
                .on_undo_builder()
                .add_entity(item_entity)
                .on_undo(|cmd, (gimmick, item)| {
                    cmd.entity(gimmick).despawn();
                    cmd.entity(item).insert(OnPickedItem);
                    cmd.insert_resource(StageStatus::playing_picked_item());
                });

            commands.entity(item_entity).remove::<OnPickedItem>();
            commands.insert_resource(StageStatus::playing_idle());
            return;
        }
    }
}


#[cfg(test)]
mod tests {
    use bevy::app::{App, Update};
    use bevy::prelude::Transform;
    use bevy_undo::prelude::{CommandsUndoExt, UndoPlugin};

    use crate::assets::gimmick::GimmickAssets;
    use crate::button::SpriteInteraction;
    use crate::page::page_index::PageIndex;
    use crate::stage::playing::gimmick::{Floor, GimmickItem, GimmickItemSpawned};
    use crate::stage::playing::gimmick::tag::GimmickTag;
    use crate::stage::playing::phase::picked_item::{OnPickedItem, spawn_item_system};
    use crate::stage::status::StageStatus;

    fn new_app() -> App {
        let mut app = App::new();
        app.add_plugins(UndoPlugin);

        app.insert_resource(PageIndex::new(0));
        app.insert_resource(GimmickAssets::default());
        app.add_systems(Update, spawn_item_system);

        app
            .world
            .spawn(GimmickItem(GimmickTag::Rock))
            .insert(OnPickedItem);

        app.world
            .spawn(Transform::from_xyz(10., 0., 0.))
            .insert(SpriteInteraction::Clicked)
            .insert(Floor);
        app
    }

    #[test]
    fn spawn_item() {
        let mut app = new_app();

        app.update();
        assert!(app
            .world
            .query::<&OnPickedItem>()
            .iter(&app.world)
            .next()
            .is_none()
        );

        // gimmick spawned
        assert!(app
            .world
            .query::<&GimmickItemSpawned>()
            .iter(&app.world)
            .next()
            .is_some()
        );

        assert_eq!(*app.world.resource::<StageStatus>(), StageStatus::playing_idle());
    }


    #[test]
    fn despawn_on_undo() {
        let mut app = new_app();

        app.update();

        app.undo();
        app.update();

        assert!(app
            .world
            .query::<&OnPickedItem>()
            .iter(&app.world)
            .next()
            .is_some()
        );
        assert!(app
            .world
            .query::<&GimmickItemSpawned>()
            .iter(&app.world)
            .next()
            .is_none()
        );

        assert_eq!(*app.world.resource::<StageStatus>(), StageStatus::playing_picked_item());
    }
}