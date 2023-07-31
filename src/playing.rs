use bevy::app::{App, Plugin, Startup, Update};
use bevy::math::Vec2;
use bevy::prelude::{any_with_component, AssetServer, Camera2dBundle, Commands, IntoSystemConfigs, Res};
use bevy_trait_query::RegisterExt;

use crate::gimmick::{floor, GimmickCollide, player, PlayerControllable, rock};
use crate::gimmick::player::Moving;
use crate::playing::idle::{Idle, update_move_input_handle};
use crate::playing::start_moving::{on_move_completed, StartMoving, update_start_moving};

pub mod idle;
pub mod start_moving;

#[derive(Default, Clone)]
pub struct PlayingPlugin;


impl Plugin for PlayingPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_component_as::<dyn PlayerControllable, GimmickCollide>()
            .add_systems(Startup, setup)
            .add_systems(Update, update_move_input_handle.run_if(any_with_component::<Idle>()))
            .add_systems(Update, update_start_moving.run_if(any_with_component::<StartMoving>()))
            .add_systems(Update, on_move_completed.run_if(any_with_component::<Moving>()));
    }
}


fn setup(
    mut commands: Commands,
    asset_sever: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());

    for x in 0..=24u8 {
        for y in 0..=14u8 {
            if x == 0 || y == 0 || x == 24 || y == 14 {
                let x = f32::from(x) * 50. - 12. * 50.;
                let y = f32::from(y) * 50. - 7. * 50.;

                rock::spawn(&mut commands, &asset_sever, Vec2::new(x, y));
            }
            let x = f32::from(x) * 50. - 12. * 50.;
            let y = f32::from(y) * 50. - 7. * 50.;

            floor::spawn(&mut commands, &asset_sever, Vec2::new(x, y));
        }
    }

    player::spawn(&mut commands);
}







