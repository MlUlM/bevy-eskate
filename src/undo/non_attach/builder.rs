use bevy::prelude::{Commands, Component, Entity};

use crate::undo::non_attach::{NonAttachUndoExecutable, UndoNonAttached};
use crate::undo::Undo;

#[derive(Component)]
pub struct UndoNonAttachedBuilder<const N: usize = 0> {
    arg1: Option<Entity>,
    arg2: Option<Entity>,
    arg3: Option<Entity>,
    arg4: Option<Entity>,
    arg5: Option<Entity>,
    arg6: Option<Entity>,
}


impl UndoNonAttachedBuilder<0> {
    #[inline]
    pub const fn new() -> UndoNonAttachedBuilder<0> {
        UndoNonAttachedBuilder {
            arg1: None,
            arg2: None,
            arg3: None,
            arg4: None,
            arg5: None,
            arg6: None,
        }
    }


    #[inline]
    pub fn add_entity(entity: Entity) -> UndoNonAttachedBuilder<1> {
        UndoNonAttachedBuilder {
            arg1: Some(entity),
            arg2: None,
            arg3: None,
            arg4: None,
            arg5: None,
            arg6: None,
        }
    }
}


impl UndoNonAttachedBuilder<1> {
    pub fn add_entity(self, entity: Entity) -> UndoNonAttachedBuilder<2> {
        UndoNonAttachedBuilder {
            arg1: self.arg1,
            arg2: Some(entity),
            arg3: None,
            arg4: None,
            arg5: None,
            arg6: None,
        }
    }


    pub fn build(self, undo: impl Fn(&mut Commands, Option<Entity>) + Send + Sync + 'static) -> Undo {
        UndoNonAttached::new(undo, self.arg1, self.arg2, self.arg3, self.arg4, self.arg5, self.arg6)
            .into_component()
    }
}


impl UndoNonAttachedBuilder<2> {
    pub fn add_entity(self, entity: Entity) -> UndoNonAttachedBuilder<3> {
        UndoNonAttachedBuilder {
            arg1: self.arg1,
            arg2: self.arg2,
            arg3: Some(entity),
            arg4: None,
            arg5: None,
            arg6: None,
        }
    }


    pub fn build(self, undo: impl Fn(&mut Commands, (Option<Entity>, Option<Entity>)) + Send + Sync + 'static) -> Undo {
        UndoNonAttached::new(undo, self.arg1, self.arg2, self.arg3, self.arg4, self.arg5, self.arg6).into_component()
    }
}


impl UndoNonAttachedBuilder<3> {
    pub fn add_entity(self, entity: Entity) -> UndoNonAttachedBuilder<4> {
        UndoNonAttachedBuilder {
            arg1: self.arg1,
            arg2: self.arg2,
            arg3: self.arg3,
            arg4: Some(entity),
            arg5: None,
            arg6: None,
        }
    }


    pub fn build(self, undo: impl NonAttachUndoExecutable<(Option<Entity>, Option<Entity>, Option<Entity>)>) -> Undo {
        UndoNonAttached::new(undo, self.arg1, self.arg2, self.arg3, self.arg4, self.arg5, self.arg6).into_component()
    }
}


impl UndoNonAttachedBuilder<4> {
    pub fn add_entity(self, entity: Entity) -> UndoNonAttachedBuilder<5> {
        UndoNonAttachedBuilder {
            arg1: self.arg1,
            arg2: self.arg2,
            arg3: self.arg3,
            arg4: self.arg4,
            arg5: Some(entity),
            arg6: None,
        }
    }


    pub fn build(self, undo: impl NonAttachUndoExecutable<(
        Option<Entity>,
        Option<Entity>,
        Option<Entity>,
        Option<Entity>
    )>) -> Undo {
        UndoNonAttached::new(undo, self.arg1, self.arg2, self.arg3, self.arg4, self.arg5, self.arg6).into_component()
    }
}


impl UndoNonAttachedBuilder<5> {
    pub fn add_entity(self, entity: Entity) -> UndoNonAttachedBuilder<6> {
        UndoNonAttachedBuilder {
            arg1: self.arg1,
            arg2: self.arg2,
            arg3: self.arg3,
            arg4: self.arg4,
            arg5: self.arg5,
            arg6: Some(entity),
        }
    }


    pub fn build(self, undo: impl NonAttachUndoExecutable<(
        Option<Entity>,
        Option<Entity>,
        Option<Entity>,
        Option<Entity>,
        Option<Entity>,
    )>) -> Undo {
        UndoNonAttached::new(undo, self.arg1, self.arg2, self.arg3, self.arg4, self.arg5, self.arg6).into_component()
    }
}


impl UndoNonAttachedBuilder<6> {
    pub fn build(self, undo: impl NonAttachUndoExecutable<(
        Option<Entity>,
        Option<Entity>,
        Option<Entity>,
        Option<Entity>,
        Option<Entity>,
        Option<Entity>,
    )>) -> Undo {
        UndoNonAttached::new(undo, self.arg1, self.arg2, self.arg3, self.arg4, self.arg5, self.arg6).into_component()
    }
}
