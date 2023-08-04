use bevy::prelude::*;
use bevy_trait_query::imports::Component;

use crate::undo::on_undo::UndoExecutable;

pub mod on_undo;
mod extension;


#[derive(Component)]
pub struct OnUndo(Box<dyn UndoExecutable>);


impl OnUndo {
    #[inline]
    pub(crate) fn new(exe: impl UndoExecutable + 'static) -> Self {
        Self(Box::new(exe))
    }


    #[inline]
    fn execute(&self, commands: &mut Commands, entity: Entity) {
        self.0.undo(&mut commands.entity(entity));
    }
}


#[derive(Debug, Default, Eq, PartialEq, Copy, Clone, Hash, Component)]
pub struct UndoIgnore;


#[derive(Debug, Default, Eq, PartialEq, Copy, Clone, Hash, Component)]
pub struct Undo;


#[derive(Debug, Default, Eq, PartialEq, Copy, Clone, Hash)]
pub struct UndoPlugin;


impl Plugin for UndoPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, undo
                .run_if(any_with_component::<OnUndo>()
                    .and_then(any_with_component::<Undo>())
                    .and_then(not(any_with_component::<UndoIgnore>()))
                ),
            );
    }
}


pub fn undo_if_input_keycode(
    input: Res<Input<KeyCode>>,
    mut commands: Commands,
) {
    if input.just_pressed(KeyCode::R) {
        commands.spawn(Undo);
    }
}


fn undo(
    mut commands: Commands,
    execute: Query<Entity, With<Undo>>,
    query: Query<(Entity, &OnUndo), With<OnUndo>>,
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
            .remove::<Undo>();
    }
}


#[cfg(test)]
mod tests {
    use bevy::app::App;
    use bevy::prelude::Entity;
    use bevy::sprite::SpriteBundle;

    use crate::undo::{Undo, UndoPlugin};
    use crate::undo::extension::CommandsExtension;
    use crate::undo::on_undo::OnUndoBuilder;

    #[test]
    fn once_undo() {
        let mut app = App::new();
        app.add_plugins(UndoPlugin);

        let id = new_entity(&mut app);
        // Undo is not executed unless UndoExecution is issued
        app.update();
        assert!(app.world.get_entity(id).is_some());

        app.world.get_entity_mut(id).unwrap().insert(Undo);
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

        app.world.spawn(Undo);
        app.update();
        assert!(app.world.get_entity(id1).is_some());
        assert!(app.world.get_entity(id2).is_none());

        app.world.spawn(Undo);
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

        app.world.spawn(Undo);
        app.update();
        assert!(app.world.get_entity(id1).is_some());
        assert!(app.world.get_entity(id2).is_none());

        let id3 = new_entity(&mut app);
        app.world.spawn(Undo);
        app.update();
        assert!(app.world.get_entity(id1).is_some());
        assert!(app.world.get_entity(id2).is_none());
        assert!(app.world.get_entity(id3).is_none());


        app.world.spawn(Undo);
        app.update();
        assert!(app.world.get_entity(id1).is_none());
        assert!(app.world.get_entity(id2).is_none());
        assert!(app.world.get_entity(id3).is_none());
    }


    #[test]
    fn non_attach() {
        let mut app = App::new();
        app.add_plugins(UndoPlugin);
        let id1 = app.world.spawn_empty().id();
        let id2 = app.world.spawn_empty().id();

        let on_undo = OnUndoBuilder::new()
            .add_entity(id2)
            .build(|cmd, id2| {
                cmd.despawn();
                cmd.commands().entity(id2).despawn();
            });
        app.world.entity_mut(id1).insert(on_undo);
        app.update();

        app.world.spawn(Undo);
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
            .on_undo(|command| {
                command.despawn();
            });

        entity.id()
    }
}