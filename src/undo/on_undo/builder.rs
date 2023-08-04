use bevy::ecs::system::EntityCommands;
use bevy::prelude::{Component, Entity};

use crate::undo::on_undo::five::Five;
use crate::undo::on_undo::four::Four;
use crate::undo::on_undo::one::One;
use crate::undo::on_undo::single::Single;
use crate::undo::on_undo::six::Six;
use crate::undo::on_undo::three::Three;
use crate::undo::on_undo::two::Two;
use crate::undo::OnUndo;

#[derive(Component, Default)]
pub struct OnUndoBuilder<const N: usize = 0> {
    entity1: Option<Entity>,
    entity2: Option<Entity>,
    entity3: Option<Entity>,
    entity4: Option<Entity>,
    entity5: Option<Entity>,
    entity6: Option<Entity>,
}


impl OnUndoBuilder<0> {
    #[inline]
    pub const fn new() -> OnUndoBuilder<0> {
        OnUndoBuilder {
            entity1: None,
            entity2: None,
            entity3: None,
            entity4: None,
            entity5: None,
            entity6: None,
        }
    }


    #[inline]
    pub fn add_entity(self, entity: Entity) -> OnUndoBuilder<1> {
        OnUndoBuilder {
            entity1: Some(entity),
            ..Default::default()
        }
    }


    #[inline]
    pub fn build(self, handler: impl Fn(&mut EntityCommands) + Send + Sync + 'static) -> OnUndo {
        Single::create(handler)
    }
}


impl OnUndoBuilder<1> {
    #[inline]
    pub fn add_entity(self, entity: Entity) -> OnUndoBuilder<2> {
        OnUndoBuilder {
            entity1: self.entity1,
            entity2: Some(entity),
            ..Default::default()
        }
    }


    #[inline]
    pub fn build(self, handler: impl Fn(&mut EntityCommands, Entity) + Send + Sync + 'static) -> OnUndo {
        One::create(self.entity1.unwrap(), handler)
    }
}


impl OnUndoBuilder<2> {
    #[inline]
    pub fn add_entity(self, entity: Entity) -> OnUndoBuilder<3> {
        OnUndoBuilder {
            entity1: self.entity1,
            entity2: self.entity2,
            entity3: Some(entity),
            ..Default::default()
        }
    }


    #[inline]
    pub fn build(self, handler: impl Fn(&mut EntityCommands, (Entity, Entity)) + Send + Sync + 'static) -> OnUndo {
        Two::create(self.entity1.unwrap(), self.entity2.unwrap(), handler)
    }
}


impl OnUndoBuilder<3> {
    #[inline]
    pub fn add_entity(self, entity: Entity) -> OnUndoBuilder<4> {
        OnUndoBuilder {
            entity1: self.entity1,
            entity2: self.entity2,
            entity3: self.entity3,
            entity4: Some(entity),
            ..Default::default()
        }
    }


    #[inline]
    pub fn build(self, handler: impl Fn(&mut EntityCommands, (Entity, Entity, Entity)) + Send + Sync + 'static) -> OnUndo {
        Three::create(self.entity1.unwrap(), self.entity2.unwrap(), self.entity3.unwrap(), handler)
    }
}


impl OnUndoBuilder<4> {
    #[inline]
    pub fn add_entity(self, entity: Entity) -> OnUndoBuilder<5> {
        OnUndoBuilder {
            entity1: self.entity1,
            entity2: self.entity2,
            entity3: self.entity3,
            entity4: self.entity4,
            entity5: Some(entity),
            ..Default::default()
        }
    }


    #[inline]
    pub fn build(self, handler: impl Fn(&mut EntityCommands, (Entity, Entity, Entity, Entity)) + Send + Sync + 'static) -> OnUndo {
        Four::create(self.entity1.unwrap(), self.entity2.unwrap(), self.entity3.unwrap(), self.entity4.unwrap(), handler)
    }
}


impl OnUndoBuilder<5> {
    #[inline]
    pub fn add_entity(self, entity: Entity) -> OnUndoBuilder<6> {
        OnUndoBuilder {
            entity1: self.entity1,
            entity2: self.entity2,
            entity3: self.entity3,
            entity4: self.entity4,
            entity5: self.entity5,
            entity6: Some(entity),
        }
    }


    #[inline]
    pub fn build(self, handler: impl Fn(&mut EntityCommands, (Entity, Entity, Entity, Entity, Entity)) + Send + Sync + 'static) -> OnUndo {
        Five::create(
            self.entity1.unwrap(),
            self.entity2.unwrap(),
            self.entity3.unwrap(),
            self.entity4.unwrap(),
            self.entity5.unwrap(),
            handler,
        )
    }
}


impl OnUndoBuilder<6> {
    #[inline]
    pub fn build(self, handler: impl Fn(&mut EntityCommands, (Entity, Entity, Entity, Entity, Entity, Entity)) + Send + Sync + 'static) -> OnUndo {
        Six::create(
            self.entity1.unwrap(),
            self.entity2.unwrap(),
            self.entity3.unwrap(),
            self.entity4.unwrap(),
            self.entity5.unwrap(),
            self.entity6.unwrap(),
            handler,
        )
    }
}
