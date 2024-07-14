use bevy::{math::Vec2, prelude::Component};

#[derive(Component)]
pub struct Player {
    pub direction: Vec2,
}

#[derive(Component)]
pub struct SelectedPlayer;

#[derive(Component)]
pub struct BallHolder;

impl Player {
    pub fn new() -> Self {
        Player {
            direction: Vec2::ZERO,
        }
    }
}
