use std::ops::{AddAssign, Deref, SubAssign};

use bevy::prelude::{Component, Resource};

#[derive(
Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Resource, Component
)]
pub struct PageIndex(pub usize);


impl AddAssign<usize> for PageIndex {
    fn add_assign(&mut self, rhs: usize) {
        *self = PageIndex::new(self.0 + rhs);
    }
}


impl SubAssign<usize> for PageIndex {
    fn sub_assign(&mut self, rhs: usize) {
        *self = PageIndex::new(self.0 - rhs);
    }
}


impl PageIndex {
    #[inline]
    pub const fn new(index: usize) -> Self {
        Self(index)
    }
}


impl Deref for PageIndex {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}