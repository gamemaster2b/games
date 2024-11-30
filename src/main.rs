//! # Games Collection
//! A collection of mini-games implemented in Rust using the Bevy Engine.
//!
//! This project serves as a learning exercise for Rust game development using the Bevy engine.
//! Currently, it implements a Pong clone with physics-based ball movement and paddle controls.
//!
//! ## Features
//! - Physics-based ball movement using bevy_rapier2d
//! - Two-player paddle controls
//! - Score detection via goals
//! - Configurable game parameters
//!
//! ## Controls
//! - Left paddle: W/S keys
//! - Right paddle: Up/Down arrow keys
//! - Reset ball: Spacebar

/* #![allow(unused)] */
#![allow(clippy::single_match)]

pub mod colors;

use bevy::{
    log::LogPlugin,
    prelude::*,
    text::BreakLineOn,
    window::{WindowMode, WindowResolution},
};
use bevy_rapier2d::{
    dynamics::RigidBody,
    plugin::{NoUserData, RapierPhysicsPlugin},
    prelude::*,
    render::RapierDebugRenderPlugin,
};
use rand::random;
use std::string::ToString;

const WINDOW_WIDTH: f32 = 1280.;
const WINDOW_HEIGHT: f32 = 720.;

/// Returns a random direction angle (in radians) within specified cone constraints.
///
/// The function ensures the ball moves in a playable direction by avoiding
/// extreme vertical angles. The returned angle is constrained within acceptable
/// ranges based on the provided cone angle.
///
/// # Arguments
/// * `cone` - The angle of the cone (in degrees) within which the direction should fall
///
/// # Returns
/// A random direction angle in radians
pub fn get_random_direction(cone: f32) -> f32 {
    let mut direction: f32 = rand::random::<f32>() * 360.;
    loop {
        match direction {
            left if (cone / 2. ..90.).contains(&left)
                || (180. + cone / 2. ..270.).contains(&left) =>
            {
                direction -= 45.
            }
            right
                if (270. ..360. - cone / 2.).contains(&right)
                    || (90. ..180. - cone / 2.).contains(&right) =>
            {
                direction += 45.
            }
            _ => {
                direction = direction.to_radians();
                return direction;
            }
        }
    }
}

fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(colors::CAMERA_CLEAR_COLOR));

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "ZeroToPong".to_string(),
            mode: WindowMode::Windowed,
            resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
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
        (
            spawn_camera,
            spawn_players,
            spawn_border,
            spawn_ball,
            spawn_score,
        ),
    );
    app.add_systems(Update, (move_paddles, detect_reset));
    app.add_systems(PostUpdate, reset_ball);

    app.add_event::<GameEvents>();

    app.run();
}

/// Spawns the camera for 2D rendering.
fn spawn_camera(mut commands: Commands) {
    info!("Spawning main camera");
    commands.spawn(Camera2dBundle::default());
}

/// Spawns the game boundaries and goals.
///
/// Creates:
/// - Top and bottom walls that the ball bounces off
/// - Left and right goals that trigger scoring events
pub fn spawn_border(mut commands: Commands) {
    info!("Spawning game boundaries");

    /// Spawns the top bound which the ball bounces of
    commands.spawn((
        SpatialBundle {
            transform: Transform::from_translation(Vec3::new(0., WINDOW_HEIGHT / 2., 0.)),
            ..Default::default()
        },
        Collider::cuboid(WINDOW_WIDTH / 2., 1.),
        Friction {
            coefficient: 0.,
            combine_rule: CoefficientCombineRule::Min,
        },
    ));
    info!("Spawned top boundary");

    /// Spawns the bottom bound which the ball bounces of
    commands.spawn((
        SpatialBundle {
            transform: Transform::from_translation(Vec3::new(0., -WINDOW_HEIGHT / 2., 0.)),
            ..Default::default()
        },
        Collider::cuboid(WINDOW_WIDTH / 2., 1.),
        Friction {
            coefficient: 0.,
            combine_rule: CoefficientCombineRule::Min,
        },
    ));
    info!("Spawned bottom boundary");

    /// Spawns the goal that the player on the right is defending
    commands.spawn((
        SpatialBundle {
            transform: Transform::from_translation(Vec3::new(WINDOW_WIDTH / 2., 0., 0.)),
            ..Default::default()
        },
        Collider::cuboid(1., WINDOW_HEIGHT / 2.),
        Player::PlayerLeft,
        Sensor,
    ));
    info!("Spawned right goal");

    /// Spawns the goal that the player on the left is defending
    commands.spawn((
        SpatialBundle {
            transform: Transform::from_translation(Vec3::new(-WINDOW_WIDTH / 2., 0., 0.)),
            ..Default::default()
        },
        Collider::cuboid(1., WINDOW_HEIGHT / 2.),
        Player::PlayerRight,
        Sensor,
    ));
    info!("Spawned left goal");
}

/// Represents a player in the game.
/// Used to identify paddle positions and scoring events.
#[derive(Component, Debug)]
enum Player {
    /// Left side player
    PlayerLeft,
    /// Right side player
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

/// Configuration for a paddle, including movement controls and velocity.
#[derive(Component, Debug)]
struct Paddle {
    /// Key for upward movement
    move_up: KeyCode,
    /// Key for downward movement
    move_down: KeyCode,
    /// Movement speed in units per second
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
const PADDLE_HEIGHT: f32 = WINDOW_HEIGHT / 5.;
const PADDLE_VELOCITY: f32 = 200.;

/// Spawns both player paddles with their initial positions and properties.
///
/// Creates two paddles:
/// - Left paddle controlled by W/S keys
/// - Right paddle controlled by Up/Down arrow keys
fn spawn_players(mut commands: Commands) {
    info!("Spawning player paddles");

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(
                -((WINDOW_WIDTH / 2.) - (PADDLE_WIDTH * 2.)),
                0.0,
                0.0,
            )),
            sprite: Sprite {
                color: colors::TILE_PLACEHOLDER,
                custom_size: Some(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
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
        Collider::cuboid(PADDLE_WIDTH / 2., PADDLE_HEIGHT / 2.),
    ));
    info!("Spawned left paddle");

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(
                (WINDOW_WIDTH / 2.) - (PADDLE_WIDTH * 2.),
                0.0,
                0.0,
            )),
            sprite: Sprite {
                color: colors::TILE_PLACEHOLDER,
                custom_size: Some(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
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
        Collider::cuboid(PADDLE_WIDTH / 2., PADDLE_HEIGHT / 2.),
    ));
    info!("Spawned right paddle");
}

/// System that handles paddle movement based on player input.
///
/// Updates paddle positions while ensuring they stay within the game boundaries.
/// Movement is time-delta based for smooth motion.
fn move_paddles(
    mut paddles: Query<(&mut Transform, &Paddle), With<Paddle>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut pos, settings) in &mut paddles {
        let old_pos = pos.translation.y;

        if input.pressed(settings.move_up) {
            pos.translation.y += settings.velocity * time.delta_seconds();
            pos.translation.y = pos.translation.y.clamp(
                -WINDOW_HEIGHT / 2. + (PADDLE_HEIGHT / 2.),
                (WINDOW_HEIGHT / 2.) - (PADDLE_HEIGHT / 2.),
            );
        }
        if input.pressed(settings.move_down) {
            pos.translation.y += -settings.velocity * time.delta_seconds();
            pos.translation.y = pos.translation.y.clamp(
                -WINDOW_HEIGHT / 2. + (PADDLE_HEIGHT / 2.),
                (WINDOW_HEIGHT / 2.) - (PADDLE_HEIGHT / 2.),
            );
        }

        if old_pos != pos.translation.y {
            trace!("Paddle moved to y: {}", pos.translation.y);
        }
    }
}

/// Marker component for the game ball
#[derive(Component, Debug)]
struct Ball;

const BALL_SIZE: f32 = PADDLE_HEIGHT * 6. / 12.;
const BALL_SPEED: f32 = PADDLE_VELOCITY * 1.0;
const BALL_DIRECTION_CONE: f32 = 30.;

/// Spawns the game ball with initial physics properties.
///
/// Creates a ball entity with:
/// - Sprite rendering
/// - Physics body and collider
/// - Initial velocity in a random direction
fn spawn_ball(mut commands: Commands, asset_server: Res<AssetServer>) {
    let direction: f32 = get_random_direction(BALL_DIRECTION_CONE);
    info!(
        "Spawning ball with initial direction: {:.2}°",
        direction.to_degrees()
    );

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

/// Game events that can occur during gameplay.
#[derive(Event, Debug)]
enum GameEvents {
    /// Event triggered when the ball needs to be reset
    ///
    /// # Arguments
    /// * `Player` - The player towards whom the ball should be launched
    ResetBall(Player),
}

/// System that detects when the ball needs to be reset.
///
/// Triggers a reset when:
/// - The spacebar is pressed (manual reset)
/// - The ball enters a goal (automatic reset)
fn detect_reset(
    input: Res<ButtonInput<KeyCode>>,
    balls: Query<&CollidingEntities, With<Ball>>,
    goals: Query<&Player, With<Sensor>>,
    mut game_events: EventWriter<GameEvents>,
) {
    if input.just_pressed(KeyCode::Space) {
        info!("Manual ball reset triggered");
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
                info!("Goal scored! Resetting ball");
                game_events.send(GameEvents::ResetBall(player.clone()));
                return;
            }
        }
    }
}

/// System that handles resetting the ball's position and velocity.
///
/// Responds to ResetBall events by:
/// - Moving the ball back to center
/// - Setting a new velocity based on which player scored
fn reset_ball(
    mut balls: Query<(&mut Transform, &mut Velocity), With<Ball>>,
    mut game_events: EventReader<GameEvents>,
) {
    for events in game_events.read() {
        match events {
            GameEvents::ResetBall(player) => {
                info!("Resetting ball position and velocity");
                for (mut ball, mut speed) in &mut balls {
                    ball.translation = Vec3::ZERO;
                    *speed = player.start_speed();
                    trace!("New ball velocity: {:?}", speed);
                }
            }
            _ => {}
        }
    }
}

fn spawn_score(mut commands: Commands) {
    info!("Spawning score display");
    commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                position_type: PositionType::Absolute,
                margin: UiRect::horizontal(Val::Auto),
                top: Val::ZERO,
                width: Val::Percent(30.),
                height: Val::Percent(20.),
                ..Default::default()
            },
            background_color: BackgroundColor(bevy::color::palettes::css::LIGHT_GRAY.into()),
            ..Default::default()
        })
        .with_children(|score| {
            score.spawn((
                TextBundle {
                    text: Text {
                        justify: JustifyText::Left,
                        linebreak_behavior: BreakLineOn::NoWrap,
                        sections: vec![TextSection {
                            value: "0".to_string(),
                            style: TextStyle {
                                font_size: 100.,
                                ..Default::default()
                            },
                        }],
                    },
                    ..Default::default()
                },
                Player::PlayerLeft,
            ));
            score.spawn(TextBundle {
                text: Text {
                    justify: JustifyText::Center,
                    linebreak_behavior: BreakLineOn::NoWrap,
                    sections: vec![TextSection {
                        value: "|".to_string(),
                        style: TextStyle {
                            font_size: 100.,
                            ..Default::default()
                        },
                    }],
                },
                ..Default::default()
            });
            score.spawn((
                TextBundle {
                    text: Text {
                        justify: JustifyText::Right,
                        linebreak_behavior: BreakLineOn::NoWrap,
                        sections: vec![TextSection {
                            value: "0".to_string(),
                            style: TextStyle {
                                font_size: 100.,
                                ..Default::default()
                            },
                        }],
                    },
                    ..Default::default()
                },
                Player::PlayerRight,
            ));
        });
    info!("Score display initialized");
}
