use bevy::{
    asset::{AssetServer, Handle},
    math::{vec2, vec3, Vec2},
    prelude::{Commands, Entity, Res},
    render::texture::Image,
    sprite::SpriteBundle,
    transform::components::Transform,
};
use bevy_rapier2d::{
    control::KinematicCharacterController, dynamics::RigidBody, geometry::Collider,
};

use crate::player::components::{Player, SelectedPlayer};

use super::components::{Team, TeamId};

const PLAYER_WIDTH: f32 = 21.0;
const PLAYER_HEIGHT: f32 = 31.0;

pub fn spawn_teams(mut commands: Commands, assets: Res<AssetServer>) {
    let formation = vec![
        vec2(-500.0, -200.0),
        vec2(-500.0, 0.0),
        vec2(-500.0, 200.0),
        vec2(-200.0, -200.0),
        vec2(-200.0, 0.0),
        vec2(-200.0, 200.0),
    ];

    let op_formation: Vec<Vec2> = formation
        .iter()
        .map(|pos| vec2(pos.x * -1.0, pos.y))
        .collect();

    let team_red = TeamId(commands.spawn(Team).id());
    let team_blue = TeamId(commands.spawn(Team).id());

    let red_player: Handle<Image> = assets.load("characterRed.png");
    let blue_player: Handle<Image> = assets.load("characterBlue.png");

    let mut spawn_player = |position: Vec2, player: Handle<Image>, team: TeamId| -> Entity {
        commands
            .spawn((
                SpriteBundle {
                    transform: Transform {
                        translation: vec3(position.x, position.y, 1.0),
                        ..Default::default()
                    },
                    texture: player,
                    ..Default::default()
                },
                team,
                RigidBody::KinematicPositionBased,
                Player::new(),
                Collider::cuboid(PLAYER_WIDTH / 2.0, PLAYER_HEIGHT / 2.0),
                KinematicCharacterController::default(),
            ))
            .id()
    };

    for position in formation {
        spawn_player(position, red_player.clone(), team_red.clone());
    }

    let mut e: Option<Entity> = None;
    for position in op_formation {
        e = Some(spawn_player(
            position,
            blue_player.clone(),
            team_blue.clone(),
        ));
    }

    if let Some(e) = e {
        commands.entity(e).insert(SelectedPlayer);
    }
}
