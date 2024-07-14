use bevy::{math::vec3, prelude::*, window::PrimaryWindow};
use bevy_rapier2d::{
    control::{KinematicCharacterController, KinematicCharacterControllerOutput},
    dynamics::ExternalImpulse,
    geometry::Collider,
    plugin::RapierContext,
};

use crate::ball::components::{Ball, BALL_RADIUS};

use super::components::{BallHolder, Player, SelectedPlayer};

pub fn player_movement(
    input: Res<ButtonInput<KeyCode>>,
    mut selected_player: Query<
        (&mut KinematicCharacterController, &Transform, &mut Player),
        With<SelectedPlayer>,
    >,
    time: Res<Time>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    if let Ok((mut kin, transform, mut player)) = selected_player.get_single_mut() {
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
        if !out_vec.xy().eq(&Vec2::ZERO) {
            player.direction = out_vec.xy();
        }
    }
}

pub fn control_ball(
    mut commands: Commands,
    ball_query: Query<Entity, With<Ball>>,
    player_contact_query: Query<
        (Option<&KinematicCharacterControllerOutput>, Entity),
        With<Player>,
    >,
    existing_selected_player_query: Query<Entity, With<SelectedPlayer>>,
    physics: Res<RapierContext>,
) {
    let ball = ball_query.get_single().unwrap();
    let op_player = player_contact_query.iter().find(|(kin, player)| {
        if let Some(kin) = kin {
            if kin.collisions.iter().any(|c| c.entity.eq(&ball)) {
                return true;
            }
        }

        if let Some(pair) = physics.contact_pair(ball, *player) {
            if pair.has_any_active_contacts() {
                return true;
            }
        }

        false
    });

    if let Some((_, entity)) = op_player {
        if let Ok(player) = existing_selected_player_query.get_single() {
            commands.entity(player).remove::<SelectedPlayer>();
        }
        commands.entity(ball).remove::<Collider>();
        commands
            .entity(entity)
            .insert(BallHolder)
            .insert(SelectedPlayer);
    }
}

pub fn dribble_ball(
    mut ball_query: Query<&mut Transform, With<Ball>>,
    player_holding_ball_query: Query<(&Transform, &Player), (With<BallHolder>, Without<Ball>)>,
) {
    let mut ball_transform = ball_query.get_single_mut().unwrap();
    let player_holding_ball = player_holding_ball_query.get_single();

    if let Ok((transform, player)) = player_holding_ball {
        let ball_translation = vec3(
            transform.translation.x,
            transform.translation.y,
            transform.translation.z,
        );

        let dir = vec3(player.direction.x, player.direction.y, 0.0);
        let dir = dir.mul_add(vec3(7.0, 7.0, 0.0), Vec3::ZERO);
        ball_transform.translation = ball_translation + dir
    }
}

pub fn shoot_ball(
    mut commands: Commands,
    ball_query: Query<Entity, With<Ball>>,
    input: Res<ButtonInput<KeyCode>>,
    player_query: Query<(&Player, Entity), (With<BallHolder>, With<SelectedPlayer>)>,
) {
    let ball = ball_query.get_single().unwrap();
    let selected_player = player_query.get_single();

    if let Ok((player, entity)) = selected_player {
        if input.pressed(KeyCode::KeyX) {
            commands
                .entity(ball)
                .insert(Collider::ball(BALL_RADIUS))
                .insert(ExternalImpulse {
                    impulse: player.direction * 100000.0,
                    torque_impulse: 14.0,
                });
            commands
                .entity(entity)
                .remove::<BallHolder>()
                .remove::<SelectedPlayer>();
        }
    }
}
