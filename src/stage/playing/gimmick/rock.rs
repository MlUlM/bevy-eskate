use bevy::asset::Handle;
use bevy::core::Name;
use bevy::math::Vec2;
use bevy::prelude::{Bundle, Commands, Image};
use bevy::sprite::SpriteBundle;

use crate::gimmick_assets::GimmickAssets;
use crate::page::page_index::PageIndex;
use crate::stage::playing::gimmick::{MoveToFront, new_gimmick_sprite_bundle};

#[derive(Bundle, Clone)]
pub struct RockBundle {
    sprite: SpriteBundle,
    collide: MoveToFront,
    page_index: PageIndex,
    name: Name,
}


impl RockBundle {
    #[inline]
    pub fn new(
        texture: Handle<Image>,
        pos: Vec2,
        page_index: PageIndex,
    ) -> Self {
        Self {
            sprite: new_gimmick_sprite_bundle(texture, pos),
            collide: MoveToFront,
            page_index,
            name: Name::new("Rock")
        }
    }
}


#[inline]
pub fn spawn(
    commands: &mut Commands,
    assets: &GimmickAssets,
    pos: Vec2,
    page_index: PageIndex,
) {
    commands.spawn(RockBundle::new(assets.rock.clone(), pos, page_index));
}


#[cfg(test)]
mod tests {
    #[test]
    fn move_to_front_of_rock() {}
}