use bevy::ecs::system::EntityCommands;
use bevy::prelude::Entity;

use crate::undo::on_undo::{UndoHandler, UndoExecutable};

use crate::undo::OnUndo;

pub(crate) struct Four {
    handler: Box<dyn UndoHandler<(Entity, Entity, Entity, Entity)>>,
    entity1: Entity,
    entity2: Entity,
    entity3: Entity,
    entity4: Entity,
}


impl Four {
    #[inline]
    pub(crate) fn create(
        entity1: Entity,
        entity2: Entity,
        entity3: Entity,
        entity4: Entity,
        handler: impl UndoHandler<(Entity, Entity, Entity, Entity)> + 'static,
    ) -> OnUndo {
        OnUndo::new(Self {
            entity1,
            entity2,
            entity3,
            entity4,
            handler: Box::new(handler),
        })
    }
}


impl UndoExecutable for Four {
    #[inline]
    fn undo(&self, commands: &mut EntityCommands) {
        self.handler.handle(commands, (self.entity1, self.entity2, self.entity3, self.entity4));
    }
}