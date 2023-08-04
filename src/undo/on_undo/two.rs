use bevy::ecs::system::EntityCommands;
use bevy::prelude::Entity;

use crate::undo::on_undo::{UndoHandler, UndoExecutable};

use crate::undo::OnUndo;

pub(crate) struct Two {
    undo: Box<dyn UndoHandler<(Entity, Entity)>>,
    entity1: Entity,
    entity2: Entity,
}


impl Two {
    #[inline]
    pub fn create(
        entity1: Entity,
        entity2: Entity,
        handler: impl UndoHandler<(Entity, Entity)>,
    ) -> OnUndo {
        OnUndo::new(Self {
            undo: Box::new(handler),
            entity1,
            entity2,
        })
    }
}


impl UndoExecutable for Two {
    #[inline]
    fn undo(&self, commands: &mut EntityCommands) {
        self.undo.handle(commands, (self.entity1, self.entity2));
    }
}