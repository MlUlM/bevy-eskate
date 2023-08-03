use bevy::ecs::system::EntityCommands;
use bevy::prelude::Component;

use crate::undo::Undo;

#[derive(Component)]
pub struct UndoAttached(pub(crate) Box<dyn Fn(&mut EntityCommands) + Send + Sync + 'static>);


impl UndoAttached {
    #[inline]
    pub fn new(undo: impl Fn(&mut EntityCommands) + Send + Sync + 'static) -> Undo {
        Undo::Attached(Self(Box::new(undo)))
    }
}