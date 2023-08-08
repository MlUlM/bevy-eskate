use std::ops::Deref;

use bevy::prelude::Resource;

#[derive(Default, Debug, PartialEq, Copy, Clone, Resource)]
pub struct PageCount(pub usize);


impl PageCount {
    #[inline]
    pub const fn new(page_count: usize) -> Self {
        Self(page_count)
    }
}


impl Deref for PageCount {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}