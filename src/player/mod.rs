use bevy::{
    app::{Startup, Update},
    prelude::Plugin,
};
use systems::{player_movement, spawn_player};

use self::systems::control_ball;

pub mod components;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement)
            .add_systems(Update, control_ball);
    }
}
