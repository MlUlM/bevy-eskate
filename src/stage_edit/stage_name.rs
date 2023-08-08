use bevy::prelude::{Component, Resource};

#[derive(Clone, Component, Resource, PartialEq, Default)]
pub struct StageName(pub String);
