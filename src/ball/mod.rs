use bevy::prelude::*;
use systems::spawn_ball;

pub mod components;
mod systems;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ball);
    }
}
