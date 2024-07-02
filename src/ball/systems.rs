use bevy::prelude::*;
use bevy_rapier2d::{
    dynamics::{Ccd, Damping, GravityScale, RigidBody},
    geometry::{Collider, ColliderMassProperties},
};

use super::components::Ball;

const BALL_SIZE: f32 = 18.0;

pub fn spawn_ball(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ball_img: Handle<Image> = asset_server.load("ball_soccer2.png");
    let ball_sprite_bundle = SpriteBundle {
        texture: ball_img,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    };

    let rigid_body = RigidBody::Dynamic;

    commands
        .spawn(Collider::cuboid(500.0, 50.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)));

    commands.spawn((
        Ball,
        ball_sprite_bundle,
        rigid_body,
        Collider::ball(BALL_SIZE / 2.0),
        ColliderMassProperties::Density(0.3),
        GravityScale(0.0),
        Damping {
            linear_damping: 1.0,
            angular_damping: 1.0,
        },
        Ccd::enabled(),
    ));
}
