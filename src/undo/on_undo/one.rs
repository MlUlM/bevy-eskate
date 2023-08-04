use bevy::ecs::system::EntityCommands;
use bevy::prelude::Entity;

use crate::undo::OnUndo;
use crate::undo::on_undo::{UndoExecutable, UndoHandler};

pub(crate) struct One {
    undo: Box<dyn UndoHandler<Entity>>,
    entity: Entity,
}


impl One {
    #[inline]
    pub fn create(
        entity: Entity,
        handler: impl UndoHandler<Entity>,
    ) -> OnUndo {
        OnUndo::new(Self {
            undo: Box::new(handler),
            entity,
        })
    }
}


impl UndoExecutable for One {
    #[inline]
    fn undo(&self, commands: &mut EntityCommands) {
        self.undo.handle(commands, self.entity);
    }
}