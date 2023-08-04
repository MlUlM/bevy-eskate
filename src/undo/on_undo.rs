use bevy::ecs::system::EntityCommands;
use bevy::prelude::Entity;

pub use builder::OnUndoBuilder;

mod builder;
mod single;
mod one;
mod two;
mod three;
mod four;
mod five;
mod six;

pub trait UndoExecutable: Send + Sync {
    fn undo(&self, commands: &mut EntityCommands);
}


pub trait UndoHandler<T>: Send + Sync + 'static
    where T: Send + Sync + 'static
{
    fn handle(&self, commands: &mut EntityCommands, entities: T);
}


impl<F> UndoHandler<()> for F
    where F: Fn(&mut EntityCommands) + Send + Sync + 'static
{
    #[inline]
    fn handle(&self, commands: &mut EntityCommands, _: ()) {
        self(commands);
    }
}


impl<F> UndoHandler<Entity> for F
    where F: Fn(&mut EntityCommands, Entity) + Send + Sync + 'static
{
    #[inline]
    fn handle(&self, commands: &mut EntityCommands, entities: Entity) {
        self(commands, entities)
    }
}


macro_rules! tuples {
    ($($entities: ty),*) => {
        impl<F> UndoHandler<($($entities),*)> for F
            where F: Fn(&mut EntityCommands, ($($entities),*)) + Send + Sync + 'static
        {
            #[inline]
            fn handle(&self, commands: &mut EntityCommands, entities: ($($entities),*)) {
                self(commands, entities)
            }
        }
    };
}

tuples!(Entity, Entity);
tuples!(Entity, Entity, Entity);
tuples!(Entity, Entity, Entity, Entity);
tuples!(Entity, Entity, Entity, Entity, Entity);
tuples!(Entity, Entity, Entity, Entity, Entity, Entity);
tuples!(Entity, Entity, Entity, Entity, Entity, Entity, Entity);
tuples!(Entity, Entity, Entity, Entity, Entity, Entity, Entity, Entity);
tuples!(Entity, Entity, Entity, Entity, Entity, Entity, Entity, Entity, Entity);
tuples!(Entity, Entity, Entity, Entity, Entity, Entity, Entity, Entity, Entity, Entity);
tuples!(Entity, Entity, Entity, Entity, Entity, Entity, Entity, Entity, Entity, Entity, Entity);
tuples!(Entity, Entity, Entity, Entity, Entity, Entity, Entity, Entity, Entity, Entity, Entity, Entity);

