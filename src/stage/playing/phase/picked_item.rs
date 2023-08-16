use bevy::app::{App, FixedUpdate, Update};
use bevy::input::Input;
use bevy::math::Vec2;
use bevy::prelude::{Color, Commands, default, Event, EventReader, EventWriter, in_state, IntoSystemConfigs, KeyCode, NextState, OnEnter, OnExit, Plugin, Query, Res, ResMut, Resource, Transform, Vec3, With};
use bevy::sprite::{Sprite, SpriteBundle};
use bevy_trait_query::imports::{Component, Entity};
use bevy_undo2::prelude::{AppUndoEx, UndoScheduler};

use crate::assets::gimmick::GimmickAssets;
use crate::button::SpriteInteraction;
use crate::gama_state::GameState;
use crate::GameCursorParams;
use crate::page::page_index::PageIndex;
use crate::stage::playing::gimmick::{Floor, GimmickItem, GimmickItemDisabled, GimmickItemSpawned};
use crate::stage::playing::gimmick::tag::GimmickTag;
use crate::stage::state::StageState;

#[derive(Resource, Default, Copy, Clone, Eq, PartialEq, Debug)]
pub struct PickItem(Option<(Entity, GimmickTag)>);


#[derive(Copy, Clone, Eq, PartialEq, Debug, Event)]
pub struct PickedItemEvent(pub Entity);


#[derive(Copy, Clone, PartialEq, Debug, Event)]
struct SpawnGimmickEvent(Vec3, Entity, GimmickTag);


#[derive(Copy, Clone, PartialEq, Debug, Event)]
struct UndoSpawnGimmickEvent {
    gimmick_entity: Entity,
    item_entity: Entity,
    tag: GimmickTag,
}


#[derive(Debug, Default, Copy, Clone, Hash, Eq, PartialEq)]
pub struct PlayingPickedItemPlugin;


impl Plugin for PlayingPickedItemPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<PickedItemEvent>()
            .add_event::<SpawnGimmickEvent>()
            .add_undo_event::<UndoSpawnGimmickEvent>()
            .init_resource::<PickItem>()
            .add_systems(OnEnter(StageState::PickedItem), stage_focus_system)
            .add_systems(OnExit(StageState::PickedItem), stage_un_focus_system)
            .add_systems(FixedUpdate, (
                pick_event_item_system,
                undo_spawn_item_event_system
            ).run_if(in_state(GameState::Stage)))
            .add_systems(Update, (
                click_floor_system,
                spawn_item_system,
                cancel_item_system
            ).run_if(in_state(StageState::PickedItem)));
    }
}


#[derive(Component)]
struct FocusScreen;


fn pick_event_item_system(
    mut state: ResMut<NextState<StageState>>,
    mut er: EventReader<PickedItemEvent>,
    mut pick_item: ResMut<PickItem>,
    mut cursor: GameCursorParams,
    assets: Res<GimmickAssets>,
    items: Query<&GimmickItem>,
) {
    for PickedItemEvent(item_entity) in er.iter().copied() {
        let Some(GimmickItem(tag)) = items.get(item_entity).ok() else { continue; };
        pick_item.0 = Some((item_entity, *tag));
        cursor.set_cursor(tag.image(&assets));
        state.set(StageState::PickedItem);
    }
}


fn stage_focus_system(mut commands: Commands) {
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
    mut state: ResMut<NextState<StageState>>,
    key: Res<Input<KeyCode>>,
) {
    if key.just_released(KeyCode::Escape) {
        state.set(StageState::Idle);
    }
}


fn stage_un_focus_system(
    mut commands: Commands,
    mut cursor: GameCursorParams,
    mut pick_item: ResMut<PickItem>,
    focus: Query<Entity, With<FocusScreen>>,
) {
    commands.entity(focus.single()).despawn();
    pick_item.0 = None;
    cursor.reset();
}


fn click_floor_system(
    mut ew: EventWriter<SpawnGimmickEvent>,
    pick_item: Res<PickItem>,
    floors: Query<(&SpriteInteraction, &Transform), With<Floor>>,
) {
    let Some((entity, tag)) = pick_item.0 else { return; };

    for (interaction, transform) in floors.iter() {
        if interaction.is_clicked() {
            ew.send(SpawnGimmickEvent(transform.translation, entity, tag));
            return;
        }
    }
}


fn spawn_item_system(
    mut state: ResMut<NextState<StageState>>,
    mut commands: Commands,
    mut er: EventReader<SpawnGimmickEvent>,
    mut cursor: GameCursorParams,
    mut scheduler: UndoScheduler<UndoSpawnGimmickEvent>,
    assets: Res<GimmickAssets>,
    page_index: Res<PageIndex>,
) {
    for SpawnGimmickEvent(spawn_pos, item_entity, tag) in er.iter().copied() {
        commands
            .entity(item_entity)
            .remove::<GimmickItem>()
            .insert(GimmickItemDisabled(tag));

        let gimmick_entity = tag
            .spawn(&mut commands, &assets, spawn_pos + Vec3::Z, *page_index)
            .insert((GimmickItemSpawned(tag), tag))
            .id();

        scheduler.register(UndoSpawnGimmickEvent { gimmick_entity, item_entity, tag });
        cursor.reset();
        state.set(StageState::Idle);
    }
}


fn undo_spawn_item_event_system(
    mut commands: Commands,
    mut er: EventReader<UndoSpawnGimmickEvent>,
) {
    for UndoSpawnGimmickEvent { gimmick_entity, item_entity, tag } in er.iter().copied() {
        println!("undo: undo_spawn_item_event_system");
        commands
            .entity(gimmick_entity)
            .despawn();

        commands
            .entity(item_entity)
            .remove::<GimmickItemDisabled>()
            .insert(GimmickItem(tag));
    }
}


#[cfg(test)]
mod tests {
    // fn new_app() -> App {
    //     let mut app = App::new();
    //     app.add_plugins(UndoPlugin);
    //     app.init_resource::<PickItem>();
    //     app.insert_resource(CursorAssets::default());
    //     app.insert_resource(PageIndex::new(0));
    //     app.insert_resource(GimmickAssets::default());
    //     app.add_systems(Update, spawn_item_system);
    //
    //     app
    //         .world
    //         .spawn(GimmickItem(GimmickTag::Rock));
    //
    //     app.world
    //         .spawn(Transform::from_xyz(10., 0., 0.))
    //         .insert(SpriteInteraction::Clicked)
    //         .insert(Floor);
    //     app
    // }
    //
    // #[test]
    // fn spawn_item() {
    //     let mut app = new_app();
    //
    //     app.update();
    //     assert!(app
    //         .world
    //         .query::<&PickItem>()
    //         .iter(&app.world)
    //         .next()
    //         .is_none()
    //     );
    //
    //     // gimmick spawned
    //     assert!(app
    //         .world
    //         .query::<&GimmickItemSpawned>()
    //         .iter(&app.world)
    //         .next()
    //         .is_some()
    //     );
    //
    //     assert_eq!(*app.world.resource::<StageState>(), StageState::playing_idle());
    // }
    //
    //
    // #[test]
    // fn despawn_on_undo() {
    //     let mut app = new_app();
    //
    //     app.update();
    //
    //     app.undo();
    //     app.update();
    //
    //     assert!(app
    //         .world
    //         .query::<&PickItem>()
    //         .iter(&app.world)
    //         .next()
    //         .is_some()
    //     );
    //     assert!(app
    //         .world
    //         .query::<&GimmickItemSpawned>()
    //         .iter(&app.world)
    //         .next()
    //         .is_none()
    //     );
    //
    //     assert_eq!(*app.world.resource::<StageState>(), StageState::playing_picked_item());
    // }
    //
    //
    // #[test]
    // fn disable_item_on_spawned_gimmick() {
    //     let mut app = new_app();
    //
    //     app.update();
    //
    //
    //     assert!(app
    //         .world
    //         .query::<&GimmickItem>()
    //         .iter(&app.world)
    //         .next()
    //         .is_none()
    //     );
    //     assert!(app
    //         .world
    //         .query::<&GimmickItemDisabled>()
    //         .iter(&app.world)
    //         .next()
    //         .is_some()
    //     );
    // }
    //
    //
    // #[test]
    // fn reactivate_gimmick_item_on_undo() {
    //     let mut app = new_app();
    //
    //     app.update();
    //     app.undo();
    //     app.update();
    //
    //     assert!(app
    //         .world
    //         .query::<&GimmickItem>()
    //         .iter(&app.world)
    //         .next()
    //         .is_some()
    //     );
    //     assert!(app
    //         .world
    //         .query::<&GimmickItemDisabled>()
    //         .iter(&app.world)
    //         .next()
    //         .is_none()
    //     );
    // }
}