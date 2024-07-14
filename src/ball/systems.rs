use bevy::{math::vec3, prelude::*, window::PrimaryWindow};
use bevy_rapier2d::{
    dynamics::{Ccd, CoefficientCombineRule, Damping, GravityScale, RigidBody},
    geometry::{Collider, Restitution},
};

use super::components::{Ball, BALL_RADIUS};

pub fn spawn_ball(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ball_img: Handle<Image> = asset_server.load("ball_soccer2.png");
    let ball_sprite_bundle = SpriteBundle {
        texture: ball_img,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    };

    commands.spawn((
        Ball,
        ball_sprite_bundle,
        Collider::ball(BALL_RADIUS),
        Restitution {
            coefficient: 1.2,
            combine_rule: CoefficientCombineRule::Min,
        },
        Damping {
            linear_damping: 1.0,
            angular_damping: 1.0,
        },
        RigidBody::Dynamic,
        GravityScale(0.0),
        Ccd::enabled(),
    ));
}

pub fn constrain_ball(
    mut ball_query: Query<&mut Transform, With<Ball>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let mut ball_transform = ball_query.get_single_mut().unwrap();
    let window = window_query.get_single().unwrap();

    let half_screen_x = window.width() / 2.0;
    let max_x = 0.0 + half_screen_x;
    let min_x = 0.0 - half_screen_x;

    let half_screen_y = window.height() / 2.0;
    let max_y = 0.0 + half_screen_y;
    let min_y = 0.0 - half_screen_y;

    let mut out_vec = ball_transform.translation;
    if ball_transform.translation.x > max_x {
        println!("hello world 1");
        out_vec.x = max_x;
    }

    if ball_transform.translation.x < min_x {
        println!("hello world 2");
        out_vec.x = min_x;
    }

    if ball_transform.translation.y > max_y {
        println!("hello world 3");
        out_vec.y = max_y;
    }

    if ball_transform.translation.y < min_y {
        println!("hello world 4");
        out_vec.y = min_y;
    }

    ball_transform.translation = out_vec;
}
