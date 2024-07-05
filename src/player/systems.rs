use std::ops::Deref;

use bevy::{math::vec3, prelude::*, window::PrimaryWindow};
use bevy_rapier2d::{
    control::{KinematicCharacterController, KinematicCharacterControllerOutput},
    dynamics::RigidBody,
    geometry::Collider,
};

use crate::ball::components::Ball;

use super::components::Player;

const PLAYER_WIDTH: f32 = 21.0;
const PLAYER_HEIGHT: f32 = 31.0;

pub fn spawn_player(mut commands: Commands, server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: vec3(-100.0, 0.0, 1.0),
                ..Default::default()
            },
            texture: server.load("characterRed.png"),
            ..Default::default()
        },
        RigidBody::KinematicPositionBased,
        Player,
        Collider::cuboid(PLAYER_WIDTH / 2.0, PLAYER_HEIGHT / 2.0),
        KinematicCharacterController::default(),
    ));
}

pub fn player_movement(
    input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut KinematicCharacterController, &Transform), With<Player>>,
    time: Res<Time>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let (mut kin, transform) = player_query.get_single_mut().unwrap();
    let mut dir = Vec3::ZERO;

    if input.pressed(KeyCode::ArrowUp) {
        dir += Vec3::new(0.0, 1.0, 0.0)
    }

    if input.pressed(KeyCode::ArrowDown) {
        dir += Vec3::new(0.0, -1.0, 0.0)
    }

    if input.pressed(KeyCode::ArrowRight) {
        dir += Vec3::new(1.0, 0.0, 0.0)
    }

    if input.pressed(KeyCode::ArrowLeft) {
        dir += Vec3::new(-1.0, 0.0, 0.0)
    }

    if dir.length() > 0.0 {
        dir = dir.normalize()
    }

    let mut out_vec = dir * 200.0 * time.delta_seconds();

    let half_screen_x = window.width() / 2.0;
    let max_x = 0.0 + half_screen_x;
    let min_x = 0.0 - half_screen_x;

    let half_screen_y = window.height() / 2.0;
    let max_y = 0.0 + half_screen_y;
    let min_y = 0.0 - half_screen_y;

    if transform.translation.x > max_x {
        out_vec.x = out_vec.x.min(0.0);
    }

    if transform.translation.x < min_x {
        out_vec.x = out_vec.x.max(0.0);
    }

    if transform.translation.y < min_y {
        out_vec.y = out_vec.y.max(0.0);
    }

    if transform.translation.y > max_y {
        out_vec.y = out_vec.y.min(0.0);
    }

    kin.translation = Some(out_vec.xy());
}

pub fn kick_ball(
    _commands: Commands,
    ball_query: Query<Entity, With<Ball>>,
    character_controller_output: Query<&KinematicCharacterControllerOutput, With<Player>>,
) {
    let ball = ball_query.get_single().unwrap();
    let output = character_controller_output.get_single().ok();
    if output.is_none() {
        return;
    }

    let collision = output.unwrap();
    let collision = Some(collision)
        .map(|o| o.collisions.iter().find(|c| c.entity.eq(&ball)))
        .unwrap();

    if let Some(c) = collision {
        println!("Ball collision: {}", c.translation_applied)
    }
}
