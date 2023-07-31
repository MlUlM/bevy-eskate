use bevy::asset::{AssetServer, Handle};
use bevy::math::Vec2;
use bevy::prelude::{Bundle, Commands, default, Image, Transform};
use bevy::sprite::SpriteBundle;

use crate::gimmick::GimmickCollide;

#[derive(Bundle, Clone)]
pub struct RockBundle {
    sprite: SpriteBundle,
    collide: GimmickCollide,
}


impl RockBundle {
    pub fn new(
        texture: Handle<Image>,
        pos: Vec2,
    ) -> Self {
        Self {
            sprite: SpriteBundle {
                texture,
                transform: Transform::from_xyz(pos.x, pos.y, 0.),
                ..default()
            },
            collide: GimmickCollide
        }
    }
}


pub fn spawn(
    commands: &mut Commands,
    asset_sever: &AssetServer,
    pos: Vec2,
) {
    let texture = asset_sever.load("gimmick/rock.png");
    commands.spawn(RockBundle::new(texture, pos));
}