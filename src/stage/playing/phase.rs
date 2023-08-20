use bevy::ecs::system::SystemParam;
use bevy::hierarchy::BuildChildren;
use bevy::prelude::{Commands, Entity, Query, Res, With};
use crate::page::page_index::PageIndex;
use crate::stage_edit::page::Field;

pub mod idle;
pub mod start_move;
pub mod next_page;
pub mod picked_item;
pub mod moving;


#[derive(SystemParam)]
pub struct FieldParams<'w, 's> {
    next_page_index: Res<'w, PageIndex>,
    fields: Query<'w, 's, (Entity, &'static PageIndex), With<Field>>,
}


impl<'w, 's> FieldParams<'w, 's> {
    pub fn add_child(&self, commands: &mut Commands, child: Entity){
        let (entity, _) = self
            .fields
            .iter()
            .find(|(_, page_index)|**page_index == *self.next_page_index)
            .unwrap();

        commands.entity(entity).add_child(child);
    }
}