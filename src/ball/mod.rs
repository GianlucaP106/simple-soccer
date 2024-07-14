use bevy::prelude::*;
use systems::{constrain_ball, spawn_ball};

pub mod components;
mod systems;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ball)
            .add_systems(Update, constrain_ball);
    }
}
