use bevy::ecs::system::EntityCommands;

use crate::undo::on_undo::{UndoHandler, UndoExecutable};
use crate::undo::OnUndo;

pub(crate) struct Single(Box<dyn UndoHandler<()>>);


impl Single {
    #[inline]
    pub fn create(undo: impl Fn(&mut EntityCommands) + Send + Sync + 'static) -> OnUndo {
        OnUndo::new(Self(Box::new(undo)))
    }
}


impl UndoExecutable for Single {
    #[inline]
    fn undo(&self, commands: &mut EntityCommands) {
        self.0.handle(commands, ());
    }
}