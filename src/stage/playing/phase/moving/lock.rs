use bevy::app::{App, Plugin, Update};
use bevy::audio::{AudioBundle};
use bevy::prelude::{AssetServer, Commands, Entity, Event, EventReader, EventWriter, in_state, IntoSystemConfigs, PlaybackSettings, Query, Res, ResMut, Transform, With};

use bevy_trait_query::imports::Component;
use bevy_undo2::prelude::{AppUndoEx, UndoScheduler};


use crate::assets::gimmick::GimmickAssets;
use crate::gama_state::GameState;
use crate::page::page_index::PageIndex;
use crate::stage::playing::gimmick::lock::{LockBundle, RequireKeys};
use crate::stage::playing::gimmick::player::Player;
use crate::stage::playing::move_direction::MoveDirection;
use crate::stage::playing::phase::moving::key::KeyCounter;
use crate::stage::playing::phase::start_move::{StartMoveDownEvent, StartMoveEvent};
use crate::stage::state::StageState;

#[derive(Event, Copy, Clone, Debug)]
pub struct LockEvent(pub Entity);

#[derive(Event, Copy, Clone, Debug)]
pub struct UnLockEvent(Entity, Transform, RequireKeys, PageIndex);

#[derive(Event, Copy, Clone, Debug)]
pub struct UndoUnLockEvent(Transform, RequireKeys, PageIndex);


pub struct MovingLockPlugin;

impl Plugin for MovingLockPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<LockEvent>()
            .add_event::<UnLockEvent>()
            .add_undo_event::<UndoUnLockEvent>()
            .add_systems(Update, (
                lock_event_system,
                unlock_event_system,
            ).run_if(in_state(StageState::Moving)))
            .add_systems(Update, undo_unlock_event_system
                .run_if(in_state(GameState::Stage)),
            );
    }
}


fn lock_event_system(
    mut er: EventReader<LockEvent>,
    mut start_move_writer: EventWriter<StartMoveEvent>,
    mut unlock_writer: EventWriter<UnLockEvent>,
    key_counter: Res<KeyCounter>,
    player: Query<&Transform, With<Player>>,
    locks: Query<(&Transform, &RequireKeys, &PageIndex)>,
) {
    for LockEvent(le) in er.iter().copied() {
        let Ok((lt, require_keys, page_index)) = locks.get(le) else { continue; };

        if require_keys.0 <= **key_counter {
            unlock_writer.send(UnLockEvent(le, *lt, *require_keys, *page_index));
        } else {
            start_move_writer.send(StartMoveEvent(MoveDirection::from_transform(player.single())));
        }
    }
}


#[derive(Component)]
struct UnlockAudioTarget;


fn unlock_event_system(
    mut commands: Commands,
    mut scheduler: UndoScheduler<UndoUnLockEvent>,
    mut unlock_reader: EventReader<UnLockEvent>,
    mut start_move_down_writer: EventWriter<StartMoveDownEvent>,
    mut key_counter: ResMut<KeyCounter>,
    asset_server: Res<AssetServer>,
) {
    for UnLockEvent(e, transform, require_keys, page_index) in unlock_reader.iter().copied() {
        commands.spawn(AudioBundle {
            source: asset_server.load("audio/unlock.ogg"),
            settings: PlaybackSettings::REMOVE,
        });
        *key_counter -= require_keys.0;
        start_move_down_writer.send(StartMoveDownEvent(transform.translation.z));
        commands.entity(e).despawn();
        scheduler.reserve(UndoUnLockEvent(transform, require_keys, page_index));
    }
}


fn undo_unlock_event_system(
    mut commands: Commands,
    mut er: EventReader<UndoUnLockEvent>,
    mut key_counter: ResMut<KeyCounter>,
    assets: Res<GimmickAssets>,
) {
    for UndoUnLockEvent(transform, require_keys, page_index) in er.iter().copied() {
        *key_counter += require_keys.0;

        commands.spawn(LockBundle::new(&assets, transform.translation, page_index));
    }
}