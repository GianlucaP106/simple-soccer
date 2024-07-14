use bevy::{app::Update, prelude::Plugin};
use systems::{dribble_ball, player_movement, shoot_ball};

use self::systems::control_ball;

pub mod components;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (player_movement, control_ball, dribble_ball, shoot_ball),
        );
    }
}
