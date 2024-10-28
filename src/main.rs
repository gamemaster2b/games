//! # GAMES
//! Games is a small project of mini-games I do wenever learning a new language.  
//! This is for Rust using the Bevy Engine  
//! [![Bevy Logo](https://bevyengine.org/assets/bevy_logo_docs.svg)](https://bevyengine.org)

#![allow(unused)]

pub mod abstractions;
pub mod colors;

use abstractions::get_random_direction;
use bevy::{
    math::VectorSpace,
    prelude::*,
    window::{WindowMode, WindowResolution},
};
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::{
    dynamics::RigidBody,
    plugin::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use std::string::ToString;

const WINDOW_WIDTH: f32 = 1344.;
const WINDOW_HIGHT: f32 = 756.;

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

    app.add_systems(Startup, (spawn_camera, spawn_players, spawn_ball));
    app.add_systems(Update, (move_paddles, (ball_collide, move_ball).chain()));

    app.run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Component)]
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

#[derive(Component)]
struct Ball(Vec2);

const BALL_SIZE: f32 = PADDLE_HIGHT * 3. / 12.;
const BALL_SPEED: f32 = PADDLE_VELOCITY * 1.0;
const BALL_DIRECTION_CONE: f32 = 60.;

fn spawn_ball(mut commands: Commands) {
    let direction: f32 = get_random_direction(BALL_DIRECTION_CONE);
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            sprite: Sprite {
                color: colors::TILE,
                custom_size: Some(Vec2::new(BALL_SIZE, BALL_SIZE)),
                ..Default::default()
            },
            ..Default::default()
        },
        Ball(Vec2::new(
            BALL_SPEED * direction.cos(),
            BALL_SPEED * direction.sin(),
        )),
        RigidBody::Dynamic,
        Collider::ball(BALL_SIZE),
    ));
}

fn move_ball(mut balls: Query<(&mut Transform, &Ball)>, time: Res<Time>) {
    for (mut ball_pos, ball_settings) in &mut balls {
        ball_pos.translation.x += ball_settings.0.x * time.delta_seconds();
        ball_pos.translation.y += ball_settings.0.y * time.delta_seconds();
    }
}
fn ball_collide(
    mut balls: Query<(&mut Transform, &mut Ball)>,
    paddles: Query<&Transform, (With<Paddle>, Without<Ball>)>,
    time: Res<Time>,
) {
    for (mut ball_pos, mut ball_settings) in &mut balls {
        for paddle_pos in &paddles {
            if ball_pos.translation.x - BALL_SIZE / 2.
                < paddle_pos.translation.x - PADDLE_WIDTH / 2.
                && ball_pos.translation.x + BALL_SIZE / 2.
                    > paddle_pos.translation.x + PADDLE_WIDTH / 2.
                && ball_pos.translation.y - BALL_SIZE / 2.
                    < paddle_pos.translation.y + PADDLE_HIGHT / 2.
                && ball_pos.translation.y + BALL_SIZE / 2.
                    > paddle_pos.translation.y - PADDLE_HIGHT / 2.
            {
                ball_settings.0.x *= -1.;
                ball_pos.translation.x += ball_settings.0.x * time.delta_seconds();
                ball_settings.0.y +=
                    BALL_SPEED * 0.7 * (ball_pos.translation.y - paddle_pos.translation.y)
                        / (PADDLE_HIGHT / 2.);
                ball_settings.0.y = ball_settings.0.y.clamp(
                    (360. - (BALL_DIRECTION_CONE / 2.)).to_radians().sin() * BALL_SPEED,
                    (BALL_DIRECTION_CONE / 2.).to_radians().sin() * BALL_SPEED,
                );
            }
        }
        if ball_pos.translation.y + BALL_SIZE / 2. > WINDOW_HIGHT / 2.
            || ball_pos.translation.y - BALL_SIZE / 2. < -WINDOW_HIGHT / 2.
        {
            ball_settings.0.y *= -1.;
            ball_pos.translation.y += ball_settings.0.y * time.delta_seconds();
        }
    }
}
