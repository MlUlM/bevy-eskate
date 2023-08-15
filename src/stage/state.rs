use bevy::prelude::States;

#[derive(Debug, PartialEq, States, Eq, Default, Copy, Clone, Hash)]
pub enum StageState {
    #[default]
    Idle,
    Moving,
    NextPage,
    Goaled
}