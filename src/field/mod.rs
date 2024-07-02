use bevy::{
    prelude::Plugin,
    render::{camera::ClearColor, color::Color},
};

pub mod components;
mod systems;

pub struct FieldPlugin;

impl Plugin for FieldPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(ClearColor(Color::rgb(0.0352, 0.494, 0.1333)));
    }
}
