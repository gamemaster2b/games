#![allow(unused)]

mod colors;

use bevy::{prelude::*, window::WindowMode};
use rand::{thread_rng, Rng};
use std::string::ToString;

fn main() {
    App::new()
        .insert_resource(ClearColor(colors::CLEAR_COLOR))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "ZeroToPong".to_string(),
                mode: WindowMode::BorderlessFullscreen,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, (spawn_camera, spawn_players_on_table, spawn_ball))
        .add_systems(Update, (move_paddles, move_ball, ball_collision))
        .run();
}

pub fn spawn_camera(mut commands: Commands) {
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

const PADDLE_X: f32 = 10.;
const PADDLE_Y: f32 = 150.;
const PADDLE_VELOCITY: f32 = 200.;

const TABLE_X: f32 = 700.;
const TABLE_Y: f32 = 500.;
fn spawn_players_on_table(mut commands: Commands) {
    commands.spawn(SpriteBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, -1.0)),
        sprite: Sprite {
            color: colors::BOARD,
            custom_size: Some(Vec2::new(TABLE_X, TABLE_Y)),
            ..Default::default()
        },
        ..Default::default()
    });
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(
                -((TABLE_X / 2.) - (PADDLE_X * 2.)),
                0.0,
                0.0,
            )),
            sprite: Sprite {
                color: colors::TILE_PLACEHOLDER,
                custom_size: Some(Vec2::new(PADDLE_X, PADDLE_Y)),
                ..Default::default()
            },
            ..Default::default()
        },
        Paddle {
            move_up: KeyCode::KeyW,
            move_down: KeyCode::KeyS,
            ..Default::default()
        },
    ));
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(
                ((TABLE_X / 2.) - (PADDLE_X * 2.)),
                0.0,
                0.0,
            )),
            sprite: Sprite {
                color: colors::TILE_PLACEHOLDER,
                custom_size: Some(Vec2::new(PADDLE_X, PADDLE_Y)),
                ..Default::default()
            },

            ..Default::default()
        },
        Paddle {
            move_up: KeyCode::ArrowUp,
            move_down: KeyCode::ArrowDown,
            ..Default::default()
        },
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
                -TABLE_Y / 2. + (PADDLE_Y / 2.),
                (TABLE_Y / 2.) - (PADDLE_Y / 2.),
            );
        }
        if input.pressed(settings.move_down) {
            pos.translation.y += -settings.velocity * time.delta_seconds();
            pos.translation.y = pos.translation.y.clamp(
                -TABLE_Y / 2. + (PADDLE_Y / 2.),
                (TABLE_Y / 2.) - (PADDLE_Y / 2.),
            );
        }
    }
}

#[derive(Component)]
struct Ball(Vec2);

const BALL_SIZE: f32 = PADDLE_Y * 3. / 12.;
const BALL_SPEED: f32 = PADDLE_VELOCITY * 1.0;

fn spawn_ball(mut commands: Commands) {
    let mut direction: f32 = (thread_rng().gen::<f32>() * 360.);
    'set_cone: loop {
        if direction > 45. && direction < 135. || direction > 225. && direction < 315. {
            direction += 45.;
        } else {
            let direction = direction.to_radians();
            break 'set_cone;
        }
    }
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
            BALL_SPEED * direction.sin(),
            BALL_SPEED * direction.cos(),
        )),
    ));
}

fn move_ball(mut balls: Query<(&mut Transform, &Ball)>, time: Res<Time>) {
    for (mut pos, settings) in &mut balls {
        pos.translation.x += settings.0.x * time.delta_seconds();
        pos.translation.y += settings.0.y * time.delta_seconds();
    }
}
fn ball_collision(
    mut balls: Query<(&mut Transform, &mut Ball)>,
    paddles: Query<&Transform, Without<Ball>>,
) {
    for (mut ball_pos, mut ball_settings) in &mut balls {
        for paddle_pos in &paddles {
            if ball_pos.translation.x - BALL_SIZE / 2. < paddle_pos.translation.x - PADDLE_X / 2.
                && ball_pos.translation.x + BALL_SIZE / 2.
                    > paddle_pos.translation.x + PADDLE_X / 2.
                && ball_pos.translation.y - BALL_SIZE / 2.
                    < paddle_pos.translation.y + PADDLE_Y / 2.
                && ball_pos.translation.y + BALL_SIZE / 2.
                    > paddle_pos.translation.y - PADDLE_Y / 2.
            {
                ball_settings.0.x *= -1.;
            }
        }
        if ball_pos.translation.y + BALL_SIZE / 2. > TABLE_Y / 2.
            || ball_pos.translation.y - BALL_SIZE / 2. < -TABLE_Y / 2.
        {
            ball_settings.0.y *= -1.;
        }
    }
}
