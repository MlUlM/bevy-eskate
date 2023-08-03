use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy_trait_query::imports::Component;

#[derive(Debug, Default, Eq, PartialEq, Copy, Clone, Hash, Component)]
pub struct UndoExecute;


#[derive(Debug, Default, Eq, PartialEq, Copy, Clone, Hash)]
pub struct UndoPlugin;


impl Plugin for UndoPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, undo
                .run_if(any_with_component::<Undo>()
                    // .and_then(any_with_component::<Undo>())
                    .and_then(any_with_component::<UndoExecute>())
                ),
            );
    }
}


#[derive(Component)]
pub struct Undo(Box<dyn Fn(&mut EntityCommands) + Send + Sync + 'static>);

impl Undo {
    pub fn new(undo: impl Fn(&mut EntityCommands) + Send + Sync + 'static) -> Self {
        Self(Box::new(undo))
    }
}


fn undo(
    mut commands: Commands,
    execute: Query<Entity, With<UndoExecute>>,
    query: Query<(Entity, &Undo, ), With<Undo>>,
) {
    if let Some((entity, Undo(undo))) = query
        .into_iter()
        .last()
    {
        let mut entity_command = commands.entity(entity);
        undo(&mut entity_command);
        entity_command.remove::<Undo>();
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

    use crate::undo::{Undo, UndoExecute, UndoPlugin};

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


    fn new_entity(app: &mut App) -> Entity {
        let mut entity = app
            .world
            .spawn_empty();
        entity
            .insert(SpriteBundle::default())
            .insert(Undo::new(|command| {
                command.despawn();
            }));

        entity.id()
    }
}