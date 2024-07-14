use bevy::prelude::{Component, Entity};

#[derive(Component)]
pub struct Team;

#[derive(Component, Clone)]
pub struct TeamId(pub Entity);
