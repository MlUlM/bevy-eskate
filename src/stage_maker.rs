use bevy::app::{App, Plugin, Update};
use bevy::prelude::{Camera2dBundle, Commands, OnEnter};
use bevy::sprite::SpriteBundle;
use bevy::utils::default;
use crate::gama_state::GameState;

#[derive(Default, Debug, PartialEq, Copy, Clone)]
pub struct StageMakerPlugin;


impl Plugin for StageMakerPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Maker), setup)
            .add_systems(Update, );
    }
}




fn setup(
    mut commands: Commands
){
    commands.spawn(Camera2dBundle::default());

    commands.spawn(settings_panel());
}


fn settings_panel() -> SettingsPanel{
    let panel = SpriteBundle{
        sprite: Default::default(),
        transform: Default::default(),
        ..default()
    };
    
    
}


pub struct SettingsPanel{
    sprite: SpriteBundle,
    
}


impl SettingsPanel {
    #[inline]
    pub fn new() -> Self{
        Self{
            sprite: SpriteBundle{
                sprite: Default::default(),
                transform: Default::default(),
                global_transform: Default::default(),
                texture: Default::default(),
                visibility: Default::default(),
                computed_visibility: Default::default(),
            }
        }
    }
}
