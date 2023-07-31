use bevy::asset::{AssetServer, Handle};
use bevy::math::Vec2;
use bevy::prelude::{Bundle, Commands, default, Image, Sprite, Transform};
use bevy::sprite::SpriteBundle;

use crate::gimmick::GIMMICK_SIZE;

#[derive(Bundle, Clone)]
pub struct FloorBundle {
    sprite: SpriteBundle,
}


impl FloorBundle {
    pub fn new(
        texture: Handle<Image>,
        pos: Vec2,
    ) -> Self {
        Self {
            sprite: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(GIMMICK_SIZE),
                    ..default()
                },
                texture,
                transform: Transform::from_xyz(pos.x, pos.y, 0.),
                ..default()
            }
        }
    }
}


pub fn spawn(
    commands: &mut Commands,
    asset_sever: &AssetServer,
    pos: Vec2,
) {
    let texture = asset_sever.load("gimmick/floor.png");
    commands.spawn(FloorBundle::new(texture, pos));
}