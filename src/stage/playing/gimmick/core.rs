use std::borrow::Cow;
use bevy::asset::Handle;
use bevy::core::Name;
use bevy::math::Vec3;
use bevy::prelude::{Bundle, Component, Image, SpriteBundle};

use crate::page::page_index::PageIndex;
use crate::stage::playing::collide::GimmickCollide;
use crate::stage::playing::gimmick::{Gimmick, new_gimmick_sprite_bundle};
use crate::stage::playing::move_position::MovePosition;

#[derive(Bundle, Clone)]
pub struct GimmickCoreBundle {
    sprite: SpriteBundle,
    name: Name,
    page_index: PageIndex,
    gimmick: Gimmick
}


impl GimmickCoreBundle {
    pub fn new(
        name: impl Into<Cow<'static, str>>,
        texture: Handle<Image>,
        pos: Vec3,
        page_index: PageIndex,
    ) -> Self {
        Self {
            sprite: new_gimmick_sprite_bundle(texture, pos),
            name: Name::new(name),
            page_index,
            gimmick: Gimmick
        }
    }
}


#[derive(Bundle, Clone)]
pub struct GimmickCollideBundle<T: Component> {
    collide: GimmickCollide,
    move_position: T,
}


impl<T> GimmickCollideBundle<T>
    where T: MovePosition + Default + Component
{
    #[inline]
    pub fn new(
        collide: GimmickCollide,
    ) -> Self {
        Self {
            move_position: T::default(),
            collide,
        }
    }
}