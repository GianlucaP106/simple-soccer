use bevy::{app::Startup, prelude::Plugin};
use systems::spawn_teams;

pub mod components;
mod systems;

pub struct TeamPlugin;

impl Plugin for TeamPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_teams);
    }
}
