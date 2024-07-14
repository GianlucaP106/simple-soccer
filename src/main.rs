use ball::BallPlugin;
use bevy::{app::App, DefaultPlugins};
use bevy_rapier2d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use field::FieldPlugin;
use initialization::InititalizationPlugin;
use player::PlayerPlugin;
use team::TeamPlugin;

mod ball;
mod field;
mod initialization;
mod player;
mod team;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(InititalizationPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(BallPlugin)
        .add_plugins(FieldPlugin)
        .add_plugins(TeamPlugin)
        .run();
}
