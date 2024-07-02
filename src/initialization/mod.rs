use bevy::{app::Startup, prelude::Plugin};
use systems::spawn_camera;

mod systems;

pub struct InititalizationPlugin;

impl Plugin for InititalizationPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_camera);
    }
}
