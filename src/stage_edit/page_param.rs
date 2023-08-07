use bevy::ecs::system::SystemParam;
use bevy::prelude::{Res, ResMut};

use crate::playing::PageIndex;
use crate::stage_edit::page_count::StageEditPageCount;

#[derive(SystemParam, Debug)]
pub struct PageParams<'w> {
    page_index: ResMut<'w, PageIndex>,
    page_count: Res<'w, StageEditPageCount>,
}


impl<'w> PageParams<'w> {
    #[inline]
    pub fn can_next_page(&self) -> bool {
        (**self.page_index + 1) < **self.page_count
    }


    #[inline]
    pub fn can_previous_page(&self) -> bool {
        0 < **self.page_index
    }

    #[inline]
    pub fn next_page(&mut self) {
        if self.can_next_page() {
            *self.page_index += 1;
        }
    }


    #[inline]
    pub fn previous_page(&mut self) {
        if self.can_previous_page() {
            *self.page_index -= 1;
        }
    }


    #[inline]
    pub fn page_count(&self) -> usize {
        **self.page_count
    }


    #[inline]
    #[allow(unused)]
    pub fn page_index(&self) -> usize {
        **self.page_index
    }
}