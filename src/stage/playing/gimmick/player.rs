use bevy::prelude::*;

use crate::assets::gimmick::GimmickAssets;
use crate::page::page_index::PageIndex;
use crate::stage::playing::gimmick::{Gimmick, GIMMICK_HEIGHT, GIMMICK_WIDTH, GimmickItemSpawned};
use crate::stage::playing::gimmick::tag::GimmickTag;

#[derive(Default, Clone, Copy, Component)]
pub struct Player;


#[derive(Default, Clone, Copy, Component)]
pub struct Movable;


#[derive(Default, Component, Copy, Clone, Debug)]
pub struct Moving;


pub fn spawn(commands: &mut Commands, assets: &GimmickAssets, pos: Vec3, page_index: PageIndex) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(GIMMICK_WIDTH, GIMMICK_HEIGHT)),
                color: Color::WHITE,
                ..default()
            },
            texture: assets.player.clone(),
            transform: Transform::from_translation(pos),
            ..default()
        })
        .insert((Movable, Gimmick(GimmickTag::Player), GimmickItemSpawned(GimmickTag::Player)))
        .insert(page_index)
        .insert(Name::new("Player"))
        .insert(Player);
}








