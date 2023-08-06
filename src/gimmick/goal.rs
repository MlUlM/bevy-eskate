use bevy::ecs::system::EntityCommands;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{AssetServer, Bundle, Commands, Transform};
use bevy::sprite::SpriteBundle;
use bevy_trait_query::imports::Component;
use crate::gimmick::{new_gimmick_sprite_bundle, move_linear, PlayerControllable};
use crate::gimmick::player::Moving;
use crate::playing::PageIndex;
use crate::playing::start_moving::MoveDirection;


#[derive(Component, Copy, Clone, PartialEq, Eq, Debug)]
pub struct Goaled;


#[derive(Component, Copy, Clone, Eq, PartialEq, Debug)]
pub struct GoalCollide;


impl PlayerControllable for GoalCollide {
    fn move_player(&self, collide_cmd: &mut EntityCommands, collide_transform: &mut Transform, player_transform: &mut Transform, direction: &MoveDirection) {
        let t = collide_transform.translation;
        move_linear(
            collide_cmd,
            player_transform,
            Vec3::new(t.x, t.y, 2.),
            |cmd| {
                cmd.remove::<Moving>();
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
}


impl GoalBundle {
    #[inline]
    pub fn new(
        asset: &AssetServer,
        pos: Vec2,
        page_index: PageIndex,
    ) -> Self {
        Self {
            sprite: new_gimmick_sprite_bundle(asset.load("gimmick/goal.png"), pos),
            collide: GoalCollide,
            page_index,
        }
    }
}


#[inline]
pub fn spawn(
    commands: &mut Commands,
    asset: &AssetServer,
    pos: Vec2,
    page_index: PageIndex,
) {
    commands.spawn(GoalBundle::new(asset, pos, page_index));
}




