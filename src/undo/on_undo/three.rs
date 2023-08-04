use bevy::ecs::system::EntityCommands;
use bevy::prelude::Entity;

use crate::undo::on_undo::{UndoHandler, UndoExecutable};
use crate::undo::OnUndo;

pub(crate) struct Three {
    handler: Box<dyn UndoHandler<(Entity, Entity, Entity)>>,
    entity1: Entity,
    entity2: Entity,
    entity3: Entity,
}


impl Three {
    #[inline]
    pub(crate) fn create(
        entity1: Entity,
        entity2: Entity,
        entity3: Entity,
        handler: impl UndoHandler<(Entity, Entity, Entity)> + 'static,
    ) -> OnUndo{
       OnUndo::new(Self{
           entity1,
           entity2,
           entity3,
           handler: Box::new(handler)
       })
    }
}


impl UndoExecutable for Three {
    #[inline]
    fn undo(&self, commands: &mut EntityCommands) {
        self.handler.handle(commands, (self.entity1, self.entity2, self.entity3));
    }
}