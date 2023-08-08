use bevy::core::Name;
use bevy::ecs::system::EntityCommands;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{Bundle, Commands, Transform};
use bevy::sprite::SpriteBundle;
use bevy_trait_query::imports::Component;

use crate::playing::gimmick::{GimmickCollide, move_linear, new_gimmick_sprite_bundle};
use crate::playing::gimmick::asset::GimmickAssets;
use crate::playing::move_direction::MoveDirection;
use crate::playing::PageIndex;
use crate::playing::phase::PlayingPhase;

#[derive(Component, Copy, Clone, PartialEq, Eq, Debug)]
pub struct Goaled;


#[derive(Component, Copy, Clone, Eq, PartialEq, Debug)]
pub struct GoalCollide;


impl GimmickCollide for GoalCollide {
    fn move_player(&self, collide_cmd: &mut EntityCommands, collide_transform: &mut Transform, player_transform: &mut Transform, _: &MoveDirection) {
        let t = collide_transform.translation;
        move_linear(
            collide_cmd,
            player_transform,
            Vec3::new(t.x, t.y, 2.),
            |cmd| {
                cmd.commands().insert_resource(PlayingPhase::Idle);
                cmd
                    .commands()
                    .spawn(Goaled);
            },
        );
    }
}


#[derive(Bundle, Clone)]
pub struct GoalBundle {
    sprite: SpriteBundle,
    collide: GoalCollide,
    page_index: PageIndex,
    name: Name,
}


impl GoalBundle {
    #[inline]
    pub fn new(
        assets: &GimmickAssets,
        pos: Vec2,
        page_index: PageIndex,
    ) -> Self {
        Self {
            sprite: new_gimmick_sprite_bundle(assets.goal.clone(), pos),
            collide: GoalCollide,
            page_index,
            name: Name::new("Goal"),
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
    commands.spawn(GoalBundle::new(assets, pos, page_index));
}




