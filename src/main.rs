#![allow(unused)]

pub mod colors;

use bevy::{prelude::*, window::WindowMode};
use rand::random;
use std::string::ToString;

fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(colors::CAMERA_CLEAR_COLOR));
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "ZeroToPong".to_string(),
            mode: WindowMode::Windowed,
            ..Default::default()
        }),
        ..Default::default()
    }));
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

const PADDLE_LENGTH: f32 = 10.;
const PADDLE_WIDTH: f32 = 150.;
const PADDLE_VELOCITY: f32 = 200.;

const TABLE_LENGTH: f32 = 700.;
const TABLE_WIDTH: f32 = 500.;

fn spawn_players(mut commands: Commands) {
    commands.spawn(SpriteBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, -1.0)),
        sprite: Sprite {
            color: colors::cyberpunk::BOARD,
            custom_size: Some(Vec2::new(TABLE_LENGTH, TABLE_WIDTH)),
            ..Default::default()
        },
        ..Default::default()
    });
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(
                -((TABLE_LENGTH / 2.) - (PADDLE_LENGTH * 2.)),
                0.0,
                0.0,
            )),
            sprite: Sprite {
                color: colors::TILE_PLACEHOLDER,
                custom_size: Some(Vec2::new(PADDLE_LENGTH, PADDLE_WIDTH)),
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
                (TABLE_LENGTH / 2.) - (PADDLE_LENGTH * 2.),
                0.0,
                0.0,
            )),
            sprite: Sprite {
                color: colors::TILE_PLACEHOLDER,
                custom_size: Some(Vec2::new(PADDLE_LENGTH, PADDLE_WIDTH)),
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
                -TABLE_WIDTH / 2. + (PADDLE_WIDTH / 2.),
                (TABLE_WIDTH / 2.) - (PADDLE_WIDTH / 2.),
            );
        }
        if input.pressed(settings.move_down) {
            pos.translation.y += -settings.velocity * time.delta_seconds();
            pos.translation.y = pos.translation.y.clamp(
                -TABLE_WIDTH / 2. + (PADDLE_WIDTH / 2.),
                (TABLE_WIDTH / 2.) - (PADDLE_WIDTH / 2.),
            );
        }
    }
}

#[derive(Component)]
struct Ball(Vec2);

const BALL_SIZE: f32 = PADDLE_WIDTH * 3. / 12.;
const BALL_SPEED: f32 = PADDLE_VELOCITY * 1.0;
const BALL_DIRECTION_CONE: f32 = 60.;

fn spawn_ball(mut commands: Commands) {
    let mut direction: f32 = random::<f32>() * 360.;
    'set_cone: loop {
        if direction > (BALL_DIRECTION_CONE / 2.) && direction < 90.
            || direction > 180. + (BALL_DIRECTION_CONE / 2.) && direction < 270.
        {
            direction -= 45.;
        } else if direction > 270. && direction < 360. - (BALL_DIRECTION_CONE / 2.)
            || direction > 90. && direction < 180. - (BALL_DIRECTION_CONE / 2.)
        {
            direction += 45.;
        } else {
            direction = direction.to_radians();
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
            BALL_SPEED * direction.cos(),
            BALL_SPEED * direction.sin(),
        )),
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
    paddles: Query<&Transform, (With<Paddle>, Without<Ball>)>
) {
    for (ball_pos, mut ball_settings) in &mut balls {
        for paddle_pos in &paddles {
            if ball_pos.translation.x - BALL_SIZE / 2.
                < paddle_pos.translation.x - PADDLE_LENGTH / 2.
                && ball_pos.translation.x + BALL_SIZE / 2.
                    > paddle_pos.translation.x + PADDLE_LENGTH / 2.
                && ball_pos.translation.y - BALL_SIZE / 2.
                    < paddle_pos.translation.y + PADDLE_WIDTH / 2.
                && ball_pos.translation.y + BALL_SIZE / 2.
                    > paddle_pos.translation.y - PADDLE_WIDTH / 2.
            {
                ball_settings.0.x *= -1.;
                ball_settings.0.y +=
                    BALL_SPEED *0.7 * (ball_pos.translation.y - paddle_pos.translation.y) / (PADDLE_WIDTH / 2.);
                ball_settings.0.y = ball_settings.0.y.clamp(
                    (360. - (BALL_DIRECTION_CONE / 2.)).to_radians().sin()*BALL_SPEED ,
                    (BALL_DIRECTION_CONE / 2.).to_radians().sin()*BALL_SPEED ,
                );
            }
        }
        if ball_pos.translation.y + BALL_SIZE / 2. > TABLE_WIDTH / 2.
            || ball_pos.translation.y - BALL_SIZE / 2. < -TABLE_WIDTH / 2.
        {
            ball_settings.0.y *= -1.;
        }
    }
}
