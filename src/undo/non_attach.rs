use bevy::prelude::{Commands, Component, Entity};

use crate::undo::Undo;

pub mod builder;

#[derive(Component)]
pub struct UndoNonAttached<T> {
    pub(crate) exe: Box<dyn NonAttachUndoExecutable<T>>,
    pub(crate) arg1: Option<Entity>,
    pub(crate) arg2: Option<Entity>,
    pub(crate) arg3: Option<Entity>,
    pub(crate) arg4: Option<Entity>,
    pub(crate) arg5: Option<Entity>,
    pub(crate) arg6: Option<Entity>,
}

impl<T> UndoNonAttached<T>
    where T: Send + Sync + 'static
{
    #[inline]
    pub(crate) fn new(
        undo: impl NonAttachUndoExecutable<T>,
        arg1: Option<Entity>,
        arg2: Option<Entity>,
        arg3: Option<Entity>,
        arg4: Option<Entity>,
        arg5: Option<Entity>,
        arg6: Option<Entity>,
    ) -> UndoNonAttached<T> {
        Self {
            exe: Box::new(undo),
            arg1,
            arg2,
            arg3,
            arg4,
            arg5,
            arg6,
        }
    }
}


impl UndoNonAttached<Option<Entity>> {
    #[inline]
    pub(crate) fn into_component(self) -> Undo {
        Undo::One(self)
    }
}


impl UndoNonAttached<(Option<Entity>, Option<Entity>)> {
    #[inline]
    pub(crate) fn into_component(self) -> Undo {
        Undo::Two(self)
    }
}


impl UndoNonAttached<(Option<Entity>, Option<Entity>, Option<Entity>)> {
    #[inline]
    pub(crate) fn into_component(self) -> Undo {
        Undo::Three(self)
    }
}


impl UndoNonAttached<(Option<Entity>, Option<Entity>, Option<Entity>, Option<Entity>)> {
    #[inline]
    pub(crate) fn into_component(self) -> Undo {
        Undo::Four(self)
    }
}


impl UndoNonAttached<(Option<Entity>, Option<Entity>, Option<Entity>, Option<Entity>, Option<Entity>)> {
    #[inline]
    pub(crate) fn into_component(self) -> Undo {
        Undo::Five(self)
    }
}


impl UndoNonAttached<(Option<Entity>, Option<Entity>, Option<Entity>, Option<Entity>, Option<Entity>, Option<Entity>)> {
    #[inline]
    pub(crate) fn into_component(self) -> Undo {
        Undo::Six(self)
    }
}


pub trait Mark {}


pub trait NonAttachUndoExecutable<T>: Send + Sync + 'static
    where T: Send + Sync + 'static
{
    fn undo(&self, commands: &mut Commands, entities: T);
}


impl Mark for Entity {}


impl<F> NonAttachUndoExecutable<Option<Entity>> for F
    where F: Fn(&mut Commands, Option<Entity>) + Send + Sync + 'static
{
    fn undo(&self, commands: &mut Commands, entities: Option<Entity>) {
        self(commands, entities)
    }
}


macro_rules! tuples {
    ($($entities: ty),*) => {
        impl Mark for ($(Option<$entities>),*){

        }

        impl<F> NonAttachUndoExecutable<($(Option<$entities>),*)> for F
            where F: Fn(&mut Commands, ($(Option<$entities>),*)) + Send + Sync + 'static
        {
            fn undo(&self, commands: &mut Commands, entities: ($(Option<$entities>),*)) {
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

