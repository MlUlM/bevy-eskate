use bevy::prelude::Component;

#[derive(Component, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum GimmickCollide {
    StopMove,
    IceBox,
    NextPage,
    Goal,
    Turn,
}