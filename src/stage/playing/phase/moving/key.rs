use std::ops::{AddAssign, Deref, SubAssign};

use bevy::app::{App, Plugin, Update};
use bevy::math::Vec3;
use bevy::prelude::{AssetServer, AudioBundle, Commands, Entity, Event, EventReader, EventWriter, in_state, IntoSystemConfigs, PlaybackSettings, PreUpdate, Query, Res, ResMut, Resource, Transform, With, Without};
use bevy_undo2::prelude::{AppUndoEx, UndoScheduler};

use crate::assets::gimmick::GimmickAssets;
use crate::gama_state::GameState;
use crate::page::page_index::PageIndex;
use crate::stage::playing::collide::GimmickCollide;
use crate::stage::playing::gimmick::key::KeyBundle;
use crate::stage::playing::gimmick::player::Player;
use crate::stage::playing::move_direction::MoveDirection;
use crate::stage::playing::phase::FieldParams;
use crate::stage::playing::phase::start_move::StartMoveEvent;
use crate::stage::state::StageState;

#[derive(Event, Debug, Copy, Clone, PartialEq)]
pub struct KeyEvent(pub Entity, pub MoveDirection);


#[derive(Event, Debug, Copy, Clone, PartialEq)]
pub struct UndoKeyEvent(Vec3, PageIndex);


#[derive(Default, Resource, Debug, Copy, Clone)]
pub struct KeyCounter(usize);


impl KeyCounter {
    #[inline(always)]
    pub fn increment(&mut self) {
        self.0 += 1;
    }


    #[inline(always)]
    pub fn decrement(&mut self) {
        self.0 = self.0.checked_sub(1).unwrap_or_default();
    }
}


impl Deref for KeyCounter {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


impl AddAssign<usize> for KeyCounter {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs;
    }
}


impl SubAssign<usize> for KeyCounter {
    fn sub_assign(&mut self, rhs: usize) {
        self.0 -= rhs;
    }
}


#[derive(Default, Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct MovingKeyPlugin;


impl Plugin for MovingKeyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<KeyEvent>()
            .add_undo_event::<UndoKeyEvent>()
            .init_resource::<KeyCounter>()
            .add_systems(PreUpdate, (
                key_event_system
            ).run_if(in_state(StageState::Moving)))
            .add_systems(Update, (
                undo_key_event_system
            ).run_if(in_state(GameState::Stage)));
    }
}


fn key_event_system(
    mut commands: Commands,
    mut scheduler: UndoScheduler<UndoKeyEvent>,
    mut er: EventReader<KeyEvent>,
    mut start_move_writer: EventWriter<StartMoveEvent>,
    mut key_counter: ResMut<KeyCounter>,
    asset_server: Res<AssetServer>,
    keys: Query<(&Transform, &PageIndex), (With<GimmickCollide>, Without<Player>)>,
) {
    for KeyEvent(ke, move_direction) in er.iter().copied() {
        let Ok((kt, key_page_index)) = keys.get(ke) else { continue; };

        commands.spawn(AudioBundle {
            source: asset_server.load("audio/key.ogg"),
            settings: PlaybackSettings::REMOVE,
        });
        key_counter.increment();
        commands.entity(ke).despawn();
        scheduler.reserve(UndoKeyEvent(kt.translation, *key_page_index));
        start_move_writer.send(StartMoveEvent(move_direction));
    }
}


fn undo_key_event_system(
    mut commands: Commands,
    mut er: EventReader<UndoKeyEvent>,
    mut key_counter: ResMut<KeyCounter>,
    assets: Res<GimmickAssets>,
    field_params: FieldParams,
) {
    for UndoKeyEvent(pos, page_index) in er.iter().copied() {
        let gimmick = commands.spawn(KeyBundle::new(&assets, pos + Vec3::Z, page_index)).id();
        field_params.add_child(&mut commands, gimmick);
        key_counter.decrement();
    }
}


#[cfg(test)]
mod tests {
    use bevy::math::Vec3;
    use bevy::prelude::{NextState, Transform};

    use crate::assets::gimmick::GimmickAssets;
    use crate::page::page_index::PageIndex;
    use crate::stage::playing::gimmick::key::KeyBundle;
    use crate::stage::playing::gimmick::player::PlayerBundle;
    use crate::stage::playing::phase::moving::key::{KeyCounter, KeyEvent, MovingKeyPlugin};
    use crate::stage::state::StageState;
    use crate::stage::tests::new_playing_app;

    #[test]
    fn key_increment() {
        let mut app = new_playing_app();
        app.world.resource_mut::<NextState<StageState>>().set(StageState::Moving);
        app.add_plugins(MovingKeyPlugin);
        let key = app.world.spawn(KeyBundle::new(&GimmickAssets::default(), Vec3::ZERO, PageIndex::default())).id();
        app.world.spawn(PlayerBundle::new(&GimmickAssets::default(), Vec3::ZERO, PageIndex::default()));
        app.world.send_event(KeyEvent(key));
        app.update();

        assert_eq!(app.world.resource::<KeyCounter>().0, 1);
        assert!(app.world.get::<Transform>(key).is_none());
    }
}