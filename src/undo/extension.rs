use bevy::ecs::system::EntityCommands;
use bevy::ecs::world::EntityMut;
use bevy::prelude::Commands;

use crate::undo::on_undo::OnUndoBuilder;

pub trait CommandsExtension {
    fn on_undo(&mut self, undo: impl Fn(&mut EntityCommands) + Send + Sync + 'static);
}


impl<'w, 's> CommandsExtension for Commands<'w, 's> {
    #[inline]
    fn on_undo(&mut self, undo: impl Fn(&mut EntityCommands) + Send + Sync + 'static) {
        self.spawn(OnUndoBuilder::new().build(undo));
    }
}


impl<'w> CommandsExtension for EntityMut<'w> {
    #[inline]
    fn on_undo(&mut self, undo: impl Fn(&mut EntityCommands) + Send + Sync + 'static) {
        self.insert(OnUndoBuilder::new().build(undo));
    }
}


impl<'w, 's, 'a> CommandsExtension for EntityCommands<'w, 's, 'a> {
    #[inline]
    fn on_undo(&mut self, undo: impl Fn(&mut EntityCommands) + Send + Sync + 'static) {
        self.insert(OnUndoBuilder::new().build(undo));
    }
}