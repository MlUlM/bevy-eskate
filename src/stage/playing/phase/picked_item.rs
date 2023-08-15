use bevy::app::App;
use bevy::input::Input;
use bevy::math::Vec2;
use bevy::prelude::{Color, Commands, Condition, default, IntoSystemConfigs, KeyCode, Plugin, Query, Res, Transform, Vec3, With};
use bevy::sprite::{Sprite, SpriteBundle};
use bevy_trait_query::imports::{Component, Entity};

use crate::assets::gimmick::GimmickAssets;
use crate::button::SpriteInteraction;
use crate::GameCursorParams;
use crate::page::page_index::PageIndex;
use crate::stage::playing::gimmick::{Floor, GimmickItem, GimmickItemDisabled, GimmickItemSpawned};
use crate::stage::playing::gimmick::tag::GimmickTag;
use crate::stage::state::StageState;

#[derive(Component, Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct OnPickedItem;


#[derive(Debug, Default, Copy, Clone, Hash, Eq, PartialEq)]
pub struct PlayingPickedItemPlugin;


impl Plugin for PlayingPickedItemPlugin {
    fn build(&self, app: &mut App) {
        // app
        //     .add_systems(
        //         Update,
        //         (spawn_item_system, cancel_item_system)
        //            ,
        //     )
        //     .add_systems(
        //         Update,
        //         stage_focus_system
        //             .run_if(in_state(GameState::Stage)
        //                 .and_then(resource_changed::<StageState>())
        //                 .and_then(resource_equals(StageState::playing_picked_item()))),
        //     )
        //     .add_systems(
        //         Update,
        //         stage_un_focus_system
        //             .run_if(in_state(GameState::Stage)
        //                 .and_then(any_with_component::<FocusScreen>())
        //                 .and_then(resource_changed::<StageState>())
        //                 .and_then(resource_equals(StageState::playing_idle()))),
        //     )
    }
}

#[derive(Component)]
struct FocusScreen;


fn stage_focus_system(
    mut cursor: GameCursorParams,
    mut commands: Commands,
    assets: Res<GimmickAssets>,
    picked: Query<&GimmickTag, With<OnPickedItem>>,
) {
    cursor.set_cursor(picked.single().image(&assets));

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(1920., 1080.)),
            color: Color::from([0.0, 0.0, 0.0, 0.3]),
            ..default()
        },

        ..default()
    })
        .insert(FocusScreen);
}


fn cancel_item_system(
    mut commands: Commands,
    mut cursor: GameCursorParams,
    key: Res<Input<KeyCode>>,
    picked_item: Query<Entity, With<OnPickedItem>>,
) {
    if key.just_released(KeyCode::Escape) {
        for item in picked_item.iter() {
            commands.entity(item).remove::<OnPickedItem>();
        }
        cursor.reset();
    }
}


fn stage_un_focus_system(
    mut commands: Commands,
    focus: Query<Entity, With<FocusScreen>>,
) {
    commands.entity(focus.single()).despawn();
}


fn spawn_item_system(
    mut commands: Commands,
    mut cursor: GameCursorParams,
    assets: Res<GimmickAssets>,
    page_index: Res<PageIndex>,
    picked_item: Query<(Entity, &GimmickItem), With<OnPickedItem>>,
    floors: Query<(&SpriteInteraction, &Transform), With<Floor>>,
) {
    let Some((item_entity, GimmickItem(tag))) = picked_item.iter().next() else { return; };

    for (interaction, transform) in floors.iter() {
        if interaction.is_clicked() {
            let gimmick_tag = *tag;
            commands
                .entity(item_entity)
                .remove::<GimmickItem>()
                .insert(GimmickItemDisabled(gimmick_tag));

            tag
                .spawn(&mut commands, &assets, transform.translation + Vec3::new(0., 0., 1.), *page_index)
                .insert(GimmickItemSpawned(*tag));
            // .on_undo_builder()
            // .add_entity(item_entity)
            // .on_undo(move |cmd, (gimmick, item)| {
            //     cmd.entity(gimmick).despawn();
            //     cmd
            //         .entity(item)
            //         .insert(OnPickedItem)
            //         .insert(GimmickItem(gimmick_tag))
            //         .remove::<GimmickItemDisabled>();
            //     cmd.insert_resource(StageStatus::playing_picked_item());
            // });

            commands.entity(item_entity).remove::<OnPickedItem>();

            cursor.reset();
            return;
        }
    }
}


#[cfg(test)]
mod tests {
    use bevy::app::{App, Update};
    use bevy::prelude::Transform;


    use crate::assets::cursor::CursorAssets;
    use crate::assets::gimmick::GimmickAssets;
    use crate::button::SpriteInteraction;
    use crate::page::page_index::PageIndex;
    use crate::stage::playing::gimmick::{Floor, GimmickItem, GimmickItemDisabled, GimmickItemSpawned};
    use crate::stage::playing::gimmick::tag::GimmickTag;
    use crate::stage::playing::phase::picked_item::{OnPickedItem, spawn_item_system};
    use crate::stage::state::StageState;

    fn new_app() -> App {
        let mut app = App::new();
        app.add_plugins(UndoPlugin);
        app.insert_resource(CursorAssets::default());
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

        assert_eq!(*app.world.resource::<StageState>(), StageState::playing_idle());
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

        assert_eq!(*app.world.resource::<StageState>(), StageState::playing_picked_item());
    }


    #[test]
    fn disable_item_on_spawned_gimmick() {
        let mut app = new_app();

        app.update();


        assert!(app
            .world
            .query::<&GimmickItem>()
            .iter(&app.world)
            .next()
            .is_none()
        );
        assert!(app
            .world
            .query::<&GimmickItemDisabled>()
            .iter(&app.world)
            .next()
            .is_some()
        );
    }


    #[test]
    fn reactivate_gimmick_item_on_undo() {
        let mut app = new_app();

        app.update();
        app.undo();
        app.update();

        assert!(app
            .world
            .query::<&GimmickItem>()
            .iter(&app.world)
            .next()
            .is_some()
        );
        assert!(app
            .world
            .query::<&GimmickItemDisabled>()
            .iter(&app.world)
            .next()
            .is_none()
        );
    }
}