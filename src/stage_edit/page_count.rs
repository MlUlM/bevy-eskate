use std::ops::Deref;

use bevy::prelude::Resource;

#[derive(Default, Debug, PartialEq, Copy, Clone, Resource)]
pub struct StageEditPageCount(pub usize);


impl StageEditPageCount {
    #[inline]
    pub const fn new(page_count: usize) -> Self {
        Self(page_count)
    }
}


impl Deref for StageEditPageCount {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}