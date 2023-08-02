use bevy::app::{App, Startup, Update};
use bevy::prelude::{Added, any_with_component, Commands, Entity, IntoSystemConfigs, Plugin, Query, ResMut, Resource};
use bevy_trait_query::imports::Component;

#[derive(Debug, Default, Eq, PartialEq, Copy, Clone, Hash, Resource)]
pub struct UndoCount(pub usize);


#[derive(Debug, Default, Eq, PartialEq, Copy, Clone, Hash, Component)]
pub struct UndoId(pub usize);


#[derive(Debug, Default, Eq, PartialEq, Copy, Clone, Hash)]
pub struct UndoPlugin;


impl Plugin for UndoPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(Update, register_id);
    }
}


#[derive(Component)]
pub struct Undo<T: Component> {
    undo: Box<dyn FnOnce(Entity, &mut T) + Send + Sync>,
}


impl<T: Component> Undo<T> {
    pub fn new(undo: impl FnOnce(Entity, &mut T) + Send + Sync + 'static) -> Undo<T> {
        Self {
            undo: Box::new(undo)
        }
    }
}


fn setup(
    mut commands: Commands
) {
    commands.insert_resource(UndoCount::default());
}


fn register_id<T: Component>(
    mut count: ResMut<UndoCount>,
    mut commands: Commands,
    query: Query<Entity, Added<Undo<T>>>,
) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .insert(UndoId(count.0));
        count.0 += 1;
    }
}