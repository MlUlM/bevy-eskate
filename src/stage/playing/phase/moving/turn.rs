use bevy::math::Quat;
use bevy::prelude::{Commands, Entity, Event, EventReader, EventWriter, In, Query, Transform, With};
use bevy_tweening::{Animator, EaseMethod, Tween, TweenCompleted};
use bevy_tweening::lens::TransformRotationLens;

use crate::stage::playing::gimmick::player::Player;
use crate::stage::playing::move_direction::MoveDirection;
use crate::stage::playing::phase::start_move::{StartMoveEvent, UndoPlayerEvent};

#[derive(Event, Debug, Copy, Clone, PartialEq)]
pub struct TurnEvent(pub Entity);


const TURN_CODE: u64 = 52123;


pub fn turn_event_system(
    mut commands: Commands,
    mut er: EventReader<TurnEvent>,
    player: Query<(Entity, &Transform), With<Player>>,
    turn: Query<&Transform>,
) -> Option<MoveDirection> {
    let TurnEvent(ce) = er.iter().next().copied()?;

    let (pe, pt) = player.single();
    commands.entity(pe).insert(UndoPlayerEvent::new(*pt));
    let ct = turn.get(ce).ok()?;
    let pd = MoveDirection::from_angle(pt.rotation.to_axis_angle().1);
    let td = MoveDirection::from_angle(ct.rotation.to_axis_angle().1);

    Some(td.turn(pd))
}


pub fn turn_pipe_system(
    In(next_dir): In<Option<MoveDirection>>,
    mut commands: Commands,
    player: Query<(Entity, &Transform), With<Player>>,
) {
    let Some(next_dir) = next_dir else { return; };
    let (pe, transform, ) = player.single();

    let start = transform.rotation;
    let end = next_dir.quat();

    commands
        .entity(pe)
        .insert(Animator::new(turn_tween(start, end).with_completed_event(TURN_CODE)));
}


#[inline]
pub fn turn_tween(start: Quat, end: Quat) -> Tween<Transform> {
    Tween::new(
        EaseMethod::Linear,
        std::time::Duration::from_millis(300),
        TransformRotationLens {
            start,
            end,
        },
    )
}


#[inline]
pub fn turn_completed(
    mut commands: Commands,
    mut er: EventReader<TweenCompleted>,
    mut start_move_writer: EventWriter<StartMoveEvent>,
    mut undo_player_writer: EventWriter<UndoPlayerEvent>,
    player: Query<(Entity, &Transform, &UndoPlayerEvent), With<Player>>,
) {
    for _ in er.iter().filter(|e| e.user_data == TURN_CODE) {
        let (pe, pt, undo_player_event) = player.single();

        start_move_writer.send(StartMoveEvent(MoveDirection::from_transform(pt)));

        commands.entity(pe).remove::<UndoPlayerEvent>();
        undo_player_writer.send(*undo_player_event);
    }
}


#[cfg(test)]
mod test {
    use std::f32::consts::PI;

    use bevy::app::{App, Update};
    use bevy::math::Quat;
    use bevy::prelude::{In, IntoSystem, Transform};
    use bevy::sprite::SpriteBundle;
    use bevy::utils::default;

    use crate::stage::playing::gimmick::player::Player;
    use crate::stage::playing::move_direction::MoveDirection;
    use crate::stage::playing::phase::moving::turn::{turn_event_system, TurnEvent};

    macro_rules! turn_test {
        ($name: ident, $player: expr, $col: expr, $expect: expr) => {
            #[test]
            fn $name() {
                let mut app = App::new();
                app.add_event::<TurnEvent>();
                app.add_systems(Update, turn_event_system.pipe(|In(dir): In<Option<MoveDirection>>| {
                    assert_eq!(dir, Some($expect));
                }));

                app.world.spawn(SpriteBundle { transform: Transform::from_rotation(Quat::from_rotation_z($player * PI)), ..default() }).insert(Player);
                let id = app.world.spawn(SpriteBundle { transform: Transform::from_rotation(Quat::from_rotation_z($col * PI)), ..default() }).id();

                app.world.send_event(TurnEvent(id));

                app.update();
            }
        }
    }

    turn_test!(up, 1.5, 0., MoveDirection::Up);
    turn_test!(left, 1.0, 0., MoveDirection::Left);
    turn_test!(right, 0., 0., MoveDirection::Right);
    turn_test!(down, 0.5, 0., MoveDirection::Down);


    turn_test!(up2, 1.5, 0.5, MoveDirection::Down);
    turn_test!(left2, 0., 0.5, MoveDirection::Left);
    turn_test!(right2, 0.5, 0.5, MoveDirection::Up);
    turn_test!(down2, 1., 0.5, MoveDirection::Right);
}