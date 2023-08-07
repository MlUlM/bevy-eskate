use bevy::asset::Handle;
use bevy::math::Vec2;
use bevy::prelude::{Bundle, Commands, Image};
use bevy::sprite::SpriteBundle;

use crate::gimmick::{MoveToFront, new_gimmick_sprite_bundle};
use crate::gimmick::asset::GimmickAssets;
use crate::playing::PageIndex;

#[derive(Bundle, Clone)]
pub struct RockBundle {
    sprite: SpriteBundle,
    collide: MoveToFront,
    page_index: PageIndex,
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