use bevy::{
    prelude::{Camera2dBundle, Commands, Query, With},
    window::{PrimaryWindow, Window},
};

pub fn spawn_camera(mut commands: Commands, _: Query<&Window, With<PrimaryWindow>>) {
    commands.spawn(Camera2dBundle::default());
}
