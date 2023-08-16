use bevy::app::{App, Plugin, Update};
use bevy::prelude::{Entity, Event, EventReader, EventWriter, in_state, IntoSystemConfigs, Query, Res, ResMut, Transform, With};

use crate::stage::playing::gimmick::lock::RequireKeys;
use crate::stage::playing::gimmick::player::Player;
use crate::stage::playing::move_direction::MoveDirection;
use crate::stage::playing::phase::moving::key::KeyCounter;
use crate::stage::playing::phase::start_move::StartMoveEvent;
use crate::stage::state::StageState;

#[derive(Event, Copy, Clone, Debug)]
pub struct LockEvent(pub Entity);

#[derive(Event, Copy, Clone, Debug)]
pub struct UnLockEvent(Entity, RequireKeys);


pub struct MovingLockPlugin;

impl Plugin for MovingLockPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<LockEvent>()
            .add_event::<UnLockEvent>()
            .add_systems(Update, (
                lock_event_system,
                unlock_event_system
            ).run_if(in_state(StageState::Moving)));
    }
}


fn lock_event_system(
    mut er: EventReader<LockEvent>,
    mut start_move_writer: EventWriter<StartMoveEvent>,
    mut unlock_writer: EventWriter<UnLockEvent>,
    key_counter: Res<KeyCounter>,
    player: Query<&Transform, With<Player>>,
    locks: Query<&RequireKeys>,
) {
    for LockEvent(le) in er.iter().copied() {
        let Ok(require_keys) = locks.get(le).copied() else { continue; };
        if require_keys.0 <= **key_counter {
            unlock_writer.send(UnLockEvent(le, require_keys));
        } else {
            start_move_writer.send(StartMoveEvent(MoveDirection::from_transform(player.single())));
        }
    }
}


fn unlock_event_system(
    mut er: EventReader<UnLockEvent>,
    mut key_counter: ResMut<KeyCounter>,
) {
    for UnLockEvent(e, require_keys) in er.iter().copied() {
        *key_counter -= require_keys.0;

        println!("unlock {e:?}");
    }
}