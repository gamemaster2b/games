//! # GAMES
//! Games is a small project of mini-games I do wenever learning a new language.  
//! This is for Rust using the Bevy Engine  
//! [![Bevy Logo](https://bevyengine.org/assets/bevy_logo_docs.svg)](https://bevyengine.org)

#![allow(unused)]
#![allow(clippy::single_match)]

pub mod colors;

use bevy::{
    prelude::*,
    window::{WindowMode, WindowResolution},
};
use bevy_rapier2d::{
    dynamics::RigidBody,
    plugin::{NoUserData, RapierPhysicsPlugin},
    prelude::*,
    render::RapierDebugRenderPlugin,
};
use games::*;
use rand::random;
use std::string::ToString;

const WINDOW_WIDTH: f32 = 1280.;
const WINDOW_HIGHT: f32 = 720.;

fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(colors::CAMERA_CLEAR_COLOR));

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "ZeroToPong".to_string(),
            mode: WindowMode::Windowed,
            resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HIGHT),
            ..Default::default()
        }),
        ..Default::default()
    }));

    app.insert_resource(RapierConfiguration {
        gravity: Vec2::ZERO,
        ..RapierConfiguration::new(1.)
    });
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default());
    #[cfg(debug_assertions)]
    app.add_plugins(RapierDebugRenderPlugin::default());

    app.add_systems(
        Startup,
        (spawn_camera, spawn_players, spawn_border, spawn_ball),
    );
    app.add_systems(Update, (move_paddles, detect_reset));
    app.add_systems(PostUpdate, reset_ball);

    app.add_event::<GameEvents>();

    app.run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

/// Spawns The Bounds of the game
pub fn spawn_border(mut commands: Commands) {
    /// Spawns the top bound which the ball bounces of
    commands.spawn((
        SpatialBundle {
            transform: Transform::from_translation(Vec3::new(0., WINDOW_HIGHT / 2., 0.)),
            ..Default::default()
        },
        Collider::cuboid(WINDOW_WIDTH / 2., 1.),
        Friction {
            coefficient: 0.,
            combine_rule: CoefficientCombineRule::Min,
        },
    ));

    /// Spawns the bottom bound which the ball bounces of
    commands.spawn((
        SpatialBundle {
            transform: Transform::from_translation(Vec3::new(0., -WINDOW_HIGHT / 2., 0.)),
            ..Default::default()
        },
        Collider::cuboid(WINDOW_WIDTH / 2., 1.),
        Friction {
            coefficient: 0.,
            combine_rule: CoefficientCombineRule::Min,
        },
    ));

    /// Spawns the goal that the player on the right is defending
    commands.spawn((
        SpatialBundle {
            transform: Transform::from_translation(Vec3::new(WINDOW_WIDTH / 2., 0., 0.)),
            ..Default::default()
        },
        Collider::cuboid(1., WINDOW_HIGHT / 2.),
        Player::PlayerLeft,
        Sensor,
    ));

    /// Spawns the goal that the player on the left is defending
    commands.spawn((
        SpatialBundle {
            transform: Transform::from_translation(Vec3::new(-WINDOW_WIDTH / 2., 0., 0.)),
            ..Default::default()
        },
        Collider::cuboid(1., WINDOW_HIGHT / 2.),
        Player::PlayerRight,
        Sensor,
    ));
}

#[derive(Component, Debug)]
enum Player {
    PlayerLeft,
    PlayerRight,
}

impl Player {
    fn start_speed(&self) -> Velocity {
        let direction = get_random_direction(BALL_DIRECTION_CONE);
        match self {
            Player::PlayerLeft => Velocity::linear(Vec2::new(
                BALL_SPEED * direction.cos().abs(),
                BALL_SPEED * direction.sin(),
            )),
            Player::PlayerRight => Velocity::linear(Vec2::new(
                BALL_SPEED * direction.cos().abs() * -1.,
                BALL_SPEED * direction.sin(),
            )),
        }
    }
}

impl Clone for Player {
    fn clone(&self) -> Player {
        match self {
            Player::PlayerLeft => Player::PlayerLeft,
            Player::PlayerRight => Player::PlayerRight,
        }
    }
}

#[derive(Component, Debug)]
struct Paddle {
    move_up: KeyCode,
    move_down: KeyCode,
    velocity: f32,
}

impl Default for Paddle {
    fn default() -> Self {
        Paddle {
            move_up: KeyCode::ArrowUp,
            move_down: KeyCode::ArrowDown,
            velocity: PADDLE_VELOCITY,
        }
    }
}

const PADDLE_WIDTH: f32 = 10.;
const PADDLE_HIGHT: f32 = WINDOW_HIGHT / 5.;
const PADDLE_VELOCITY: f32 = 200.;

fn spawn_players(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(
                -((WINDOW_WIDTH / 2.) - (PADDLE_WIDTH * 2.)),
                0.0,
                0.0,
            )),
            sprite: Sprite {
                color: colors::TILE_PLACEHOLDER,
                custom_size: Some(Vec2::new(PADDLE_WIDTH, PADDLE_HIGHT)),
                ..Default::default()
            },
            ..Default::default()
        },
        Paddle {
            move_up: KeyCode::KeyW,
            move_down: KeyCode::KeyS,
            ..Default::default()
        },
        RigidBody::KinematicPositionBased,
        Collider::cuboid(PADDLE_WIDTH / 2., PADDLE_HIGHT / 2.),
    ));
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(
                (WINDOW_WIDTH / 2.) - (PADDLE_WIDTH * 2.),
                0.0,
                0.0,
            )),
            sprite: Sprite {
                color: colors::TILE_PLACEHOLDER,
                custom_size: Some(Vec2::new(PADDLE_WIDTH, PADDLE_HIGHT)),
                ..Default::default()
            },

            ..Default::default()
        },
        Paddle {
            move_up: KeyCode::ArrowUp,
            move_down: KeyCode::ArrowDown,
            ..Default::default()
        },
        RigidBody::KinematicPositionBased,
        Collider::cuboid(PADDLE_WIDTH / 2., PADDLE_HIGHT / 2.),
    ));
}
fn move_paddles(
    mut paddles: Query<(&mut Transform, &Paddle), With<Paddle>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut pos, settings) in &mut paddles {
        if input.pressed(settings.move_up) {
            pos.translation.y += settings.velocity * time.delta_seconds();
            pos.translation.y = pos.translation.y.clamp(
                -WINDOW_HIGHT / 2. + (PADDLE_HIGHT / 2.),
                (WINDOW_HIGHT / 2.) - (PADDLE_HIGHT / 2.),
            );
        }
        if input.pressed(settings.move_down) {
            pos.translation.y += -settings.velocity * time.delta_seconds();
            pos.translation.y = pos.translation.y.clamp(
                -WINDOW_HIGHT / 2. + (PADDLE_HIGHT / 2.),
                (WINDOW_HIGHT / 2.) - (PADDLE_HIGHT / 2.),
            );
        }
    }
}

#[derive(Component, Debug)]
struct Ball;

const BALL_SIZE: f32 = PADDLE_HIGHT * 6. / 12.;
const BALL_SPEED: f32 = PADDLE_VELOCITY * 1.0;
const BALL_DIRECTION_CONE: f32 = 30.;

fn spawn_ball(mut commands: Commands, asset_server: Res<AssetServer>) {
    let direction: f32 = get_random_direction(BALL_DIRECTION_CONE);
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("bevy.png"),
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            sprite: Sprite {
                custom_size: Some(Vec2::new(BALL_SIZE, BALL_SIZE)),
                ..Default::default()
            },
            ..Default::default()
        },
        Ball,
        RigidBody::Dynamic,
        ActiveEvents::COLLISION_EVENTS,
        Collider::ball(BALL_SIZE / 2.),
        CollidingEntities::default(),
        Velocity::linear(Vec2::new(
            BALL_SPEED * direction.cos(),
            BALL_SPEED * direction.sin(),
        )),
        Restitution {
            coefficient: 1.1,
            combine_rule: CoefficientCombineRule::Max,
        },
    ));
}

#[derive(Event, Debug)]
enum GameEvents {
    ResetBall(Player),
}

fn detect_reset(
    input: Res<ButtonInput<KeyCode>>,
    balls: Query<&CollidingEntities, With<Ball>>,
    goals: Query<&Player, With<Sensor>>,
    mut game_events: EventWriter<GameEvents>,
) {
    if input.just_pressed(KeyCode::Space) {
        let player = if random::<bool>() {
            Player::PlayerLeft
        } else {
            Player::PlayerRight
        };
        game_events.send(GameEvents::ResetBall(player));
        return;
    }
    for ball in &balls {
        for hit in ball.iter() {
            if let Ok(player) = goals.get(hit) {
                game_events.send(GameEvents::ResetBall(player.clone()));
                return;
            }
        }
    }
}

fn reset_ball(
    mut balls: Query<(&mut Transform, &mut Velocity), With<Ball>>,
    mut game_events: EventReader<GameEvents>,
) {
    for events in game_events.read() {
        match events {
            GameEvents::ResetBall(player) => {
                for (mut ball, mut speed) in &mut balls {
                    ball.translation = Vec3::ZERO;
                    *speed = player.start_speed();
                }
            }
            _ => {}
        }
    }
}
