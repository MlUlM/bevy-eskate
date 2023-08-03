use bevy::prelude::*;
use bevy_trait_query::imports::Component;

use crate::undo::attached::UndoAttached;
use crate::undo::non_attach::UndoNonAttached;

mod non_attach;
pub mod attached;

#[derive(Component)]
pub enum Undo {
    Attached(UndoAttached),
    One(UndoNonAttached<Option<Entity>>),
    Two(UndoNonAttached<(Option<Entity>, Option<Entity>)>),
    Three(UndoNonAttached<(Option<Entity>, Option<Entity>, Option<Entity>)>),
    Four(UndoNonAttached<(Option<Entity>, Option<Entity>, Option<Entity>, Option<Entity>)>),
    Five(UndoNonAttached<(Option<Entity>, Option<Entity>, Option<Entity>, Option<Entity>, Option<Entity>)>),
    Six(UndoNonAttached<(Option<Entity>, Option<Entity>, Option<Entity>, Option<Entity>, Option<Entity>, Option<Entity>)>),
}


impl Undo {
    fn execute(&self, commands: &mut Commands, entity: Entity) {
        match self {
            Undo::Attached(UndoAttached(f)) => { f(&mut commands.entity(entity)) }
            Undo::One(undo) => {
                let args1 = args(commands, Some(undo.arg1));
                undo.exe.undo(commands, args1)
            }

            Undo::Two(undo) => {
                let args1 = args(commands, Some(undo.arg1));
                let args2 = args(commands, undo.arg2);
                undo.exe.undo(commands, (args1, args2))
            }

            Undo::Three(undo) => {
                let args1 = args(commands, Some(undo.arg1));
                let args2 = args(commands, undo.arg2);
                let args3 = args(commands, undo.arg3);

                undo.exe.undo(commands, (
                    args1,
                    args2,
                    args3
                ));
            }

            Undo::Four(undo) => {
                let args1 = args(commands, Some(undo.arg1));
                let args2 = args(commands, undo.arg2);
                let args3 = args(commands, undo.arg3);
                let args4 = args(commands, undo.arg4);

                undo.exe.undo(commands, (
                    args1,
                    args2,
                    args3,
                    args4
                ));
            }

            Undo::Five(undo) => {
                let args1 = args(commands, Some(undo.arg1));
                let args2 = args(commands, undo.arg2);
                let args3 = args(commands, undo.arg3);
                let args4 = args(commands, undo.arg4);
                let args5 = args(commands, undo.arg5);

                undo.exe.undo(commands, (
                    args1,
                    args2,
                    args3,
                    args4,
                    args5
                ));
            }

            Undo::Six(undo) => {
                let args1 = args(commands, Some(undo.arg1));
                let args2 = args(commands, undo.arg2);
                let args3 = args(commands, undo.arg3);
                let args4 = args(commands, undo.arg4);
                let args5 = args(commands, undo.arg5);
                let args6 = args(commands, undo.arg6);

                undo.exe.undo(commands, (
                    args1,
                    args2,
                    args3,
                    args4,
                    args5,
                    args6
                ));
            }
        }
    }
}


fn args(commands: &mut Commands, expect_entity: Option<Entity>) -> Option<Entity> {
    let e = expect_entity?;

    if commands.get_entity(e).is_some() {
        Some(e)
    } else {
        None
    }
}

#[derive(Debug, Default, Eq, PartialEq, Copy, Clone, Hash, Component)]
pub struct UndoExecute;


#[derive(Debug, Default, Eq, PartialEq, Copy, Clone, Hash)]
pub struct UndoPlugin;


impl Plugin for UndoPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, undo
                .run_if(any_with_component::<Undo>()
                    .and_then(any_with_component::<UndoExecute>())
                ),
            );
    }
}


pub fn undo_if_input_keycode(
    input: Res<Input<KeyCode>>,
    mut commands: Commands,
) {
    if input.just_pressed(KeyCode::R) {
        commands.spawn(UndoExecute);
    }
}


fn undo(
    mut commands: Commands,
    execute: Query<Entity, With<UndoExecute>>,
    query: Query<(
        Entity,
        &Undo
    ), With<Undo>>,
) {
    if let Some((entity, undo)) = query
        .into_iter()
        .last()
    {
        undo.execute(&mut commands, entity);
    }

    for exe in execute.iter() {
        commands
            .entity(exe)
            .remove::<UndoExecute>();
    }
}


#[cfg(test)]
mod tests {
    use bevy::app::App;
    use bevy::prelude::Entity;
    use bevy::sprite::SpriteBundle;

    use crate::undo::{UndoExecute, UndoPlugin};
    use crate::undo::attached::UndoAttached;
    use crate::undo::non_attach::builder::UndoNonAttachedBuilder;

    #[test]
    fn once_undo() {
        let mut app = App::new();
        app.add_plugins(UndoPlugin);

        let id = new_entity(&mut app);
        // Undo is not executed unless UndoExecution is issued
        app.update();
        assert!(app.world.get_entity(id).is_some());

        app.world.get_entity_mut(id).unwrap().insert(UndoExecute);
        app.update();
        assert!(app.world.get_entity(id).is_none());
    }


    #[test]
    fn two_undo() {
        let mut app = App::new();
        app.add_plugins(UndoPlugin);

        let id1 = new_entity(&mut app);
        let id2 = new_entity(&mut app);

        // Undo is not executed unless UndoExecution is issued
        app.update();
        // Two frames are consumed before Undo is monitored for ID registration.
        // app.update();
        assert!(app.world.get_entity(id1).is_some());
        assert!(app.world.get_entity(id2).is_some());

        app.world.spawn(UndoExecute);
        app.update();
        assert!(app.world.get_entity(id1).is_some());
        assert!(app.world.get_entity(id2).is_none());

        app.world.spawn(UndoExecute);
        app.update();
        assert!(app.world.get_entity(id1).is_none());
        assert!(app.world.get_entity(id2).is_none());
    }


    #[test]
    fn three_undo() {
        let mut app = App::new();
        app.add_plugins(UndoPlugin);

        let id1 = new_entity(&mut app);
        let id2 = new_entity(&mut app);

        app.world.spawn(UndoExecute);
        app.update();
        assert!(app.world.get_entity(id1).is_some());
        assert!(app.world.get_entity(id2).is_none());

        let id3 = new_entity(&mut app);
        app.world.spawn(UndoExecute);
        app.update();
        assert!(app.world.get_entity(id1).is_some());
        assert!(app.world.get_entity(id2).is_none());
        assert!(app.world.get_entity(id3).is_none());


        app.world.spawn(UndoExecute);
        app.update();
        assert!(app.world.get_entity(id1).is_none());
        assert!(app.world.get_entity(id2).is_none());
        assert!(app.world.get_entity(id3).is_none());
    }


    #[test]
    fn non_attach() {
        let mut app = App::new();
        app.add_plugins(UndoPlugin);
        let id1 = new_entity(&mut app);
        let id2 = new_entity(&mut app);

        let undo = UndoNonAttachedBuilder::new(id1)
            .add_entity(id2)
            .build(|cmd, (id1, id2)| {
                cmd.entity(id1.unwrap()).despawn();
                cmd.entity(id2.unwrap()).despawn();
            });
        app.world.spawn(undo);
        app.update();

        app.world.spawn(UndoExecute);
        app.update();
        assert!(app.world.get_entity(id1).is_none());
        assert!(app.world.get_entity(id2).is_none());
    }


    fn new_entity(app: &mut App) -> Entity {
        let mut entity = app
            .world
            .spawn_empty();
        entity
            .insert(SpriteBundle::default())
            .insert(UndoAttached::new(|command| {
                command.despawn();
            }));

        entity.id()
    }
}