use bevy::app::{App, Plugin, Update};
use bevy::input::Input;
use bevy::prelude::*;

use crate::extension::InteractionCondition;
use crate::mouse_just_pressed_left;
use crate::page::page_index::PageIndex;
use crate::stage::playing::gimmick::{GimmickItem, GimmickItemDisabled};
use crate::stage::playing::move_direction::MoveDirection;
use crate::stage::playing::phase::picked_item::PickedItemEvent;
use crate::stage::playing::phase::start_move::StartMoveEvent;
use crate::stage::state::StageState;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct PlayingIdlePlugin;


impl Plugin for PlayingIdlePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update,
                (
                    update_item_colors_system,
                    input_move_system,
                    picked_item_system.run_if(mouse_just_pressed_left)
                )
                    .run_if(in_state(StageState::Idle)),
            );
    }
}


fn input_move_system(
    mut ew: EventWriter<StartMoveEvent>,
    keys: Res<Input<KeyCode>>,
) {
    let mut emit = |direction: MoveDirection| {
        println!("send -> {:?}", StartMoveEvent(direction));
        ew.send(StartMoveEvent(direction));
    };

    if keys.any_just_pressed([KeyCode::Left, KeyCode::A]) {
        emit(MoveDirection::Left);
    } else if keys.any_just_pressed([KeyCode::Up, KeyCode::W]) {
        emit(MoveDirection::Up);
    } else if keys.any_just_pressed([KeyCode::Right, KeyCode::D]) {
        emit(MoveDirection::Right);
    } else if keys.any_just_pressed([KeyCode::Down, KeyCode::S]) {
        emit(MoveDirection::Down);
    }
}


fn picked_item_system(
    mut ew: EventWriter<PickedItemEvent>,
    page_index: Res<PageIndex>,
    items: Query<(Entity, &Interaction, &GimmickItem, &PageIndex)>,
) {
    for (item_entity, interaction, GimmickItem(_), _) in items
        .iter()
        .filter(|(_, _, _, idx)| **idx == *page_index)
    {
        if interaction.pressed() {
            ew.send(PickedItemEvent(item_entity));

            return;
        }
    }
}


fn update_item_colors_system(
    mut active_items: Query<&mut BackgroundColor, (Added<GimmickItem>, Without<GimmickItemDisabled>)>,
    mut deactive_items: Query<&mut BackgroundColor, (Added<GimmickItemDisabled>, Without<GimmickItem>)>,
) {
    for mut item in active_items.iter_mut() {
        *item = BackgroundColor::default();
    }

    for mut item in deactive_items.iter_mut() {
        *item = BackgroundColor::from(Color::GRAY);
    }
}


#[cfg(test)]
mod tests {
    use bevy::app::{App, Update};
    use bevy::input::Input;
    use bevy::prelude::{Commands, EventReader, IntoSystemConfigs, KeyCode};

    use crate::stage::playing::move_direction::MoveDirection;
    use crate::stage::playing::phase::idle::input_move_system;
    use crate::stage::playing::phase::start_move::StartMoveEvent;
    use crate::stage::tests::new_playing_app;

    #[test]
    fn input_keycodes() {
        let mut app = new_playing_app();
        app.add_systems(Update, (input_move_system, read).chain());

        input(&mut app, KeyCode::Left, MoveDirection::Left);
        input(&mut app, KeyCode::Up, MoveDirection::Up);
        input(&mut app, KeyCode::Right, MoveDirection::Right);
        input(&mut app, KeyCode::Down, MoveDirection::Down);
    }


    fn input(app: &mut App, key: KeyCode, expect: MoveDirection) {
        let mut input = Input::<KeyCode>::default();
        input.press(key);
        app.insert_resource(input);

        app.update();

        assert!(app.world.query::<&MoveDirection>().iter(&app.world).any(|d|*d == expect));
    }


    fn read(mut commands: Commands, mut er: EventReader<StartMoveEvent>) {
        for StartMoveEvent(dir) in er.iter().copied() {
            commands.spawn(dir);
        }
    }
}