use bevy::prelude::*;

use crate::gimmick::{Gimmick, GimmickItem};
use crate::gimmick::tag::GimmickTag;
use crate::playing::PageIndex;

#[derive(Default, Clone, Copy, Component)]
pub struct Movable;


#[derive(Default, Component, Copy, Clone, Debug)]
pub struct Moving;


pub fn spawn(commands: &mut Commands, asset: &AssetServer, pos: Vec2, page_index: PageIndex) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(50., 50.)),
                color: Color::WHITE,
                ..default()
            },
            texture: asset.load("gimmick/player.png"),
            transform: Transform::from_xyz(pos.x, pos.y, 1.),
            ..default()
        })
        .insert((Movable, Gimmick(GimmickTag::Player), GimmickItem(GimmickTag::Player)))
        .insert(page_index);
}








