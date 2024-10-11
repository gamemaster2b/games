#![allow(unused)]

mod colors;

use bevy::{prelude::*, window::WindowMode};
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
        .add_systems(Startup, (spawn_camera, spawn_players))
        .add_systems(Update, move_puddles)
        .run();
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Component)]
struct Puddle {
    move_up: KeyCode,
    move_down: KeyCode,
}

const PUDDLE_X: f32 = 10.;
const PUDDLE_Y: f32 = 150.;
fn spawn_players(mut commands: Commands) {
    commands.spawn(SpriteBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, -1.0)),
        sprite: Sprite {
            color: colors::BOARD,
            custom_size: Some(Vec2::new(700., 500.)),
            ..Default::default()
        },
        ..Default::default()
    });
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(-300.0, 0.0, 0.0)),
            sprite: Sprite {
                color: colors::TILE_PLACEHOLDER,
                custom_size: Some(Vec2::new(PUDDLE_X, PUDDLE_Y)),
                ..Default::default()
            },
            ..Default::default()
        },
        Puddle {
            move_up: KeyCode::KeyW,
            move_down: KeyCode::KeyS,
        },
    ));
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(300.0, 0.0, 0.0)),
            sprite: Sprite {
                color: colors::TILE_PLACEHOLDER,
                custom_size: Some(Vec2::new(PUDDLE_X, PUDDLE_Y)),
                ..Default::default()
            },

            ..Default::default()
        },
        Puddle {
            move_up: KeyCode::ArrowUp,
            move_down: KeyCode::ArrowDown,
        },
    ));
}
fn move_puddles(
    mut paddles: Query<(&mut Transform, &Puddle), With<Puddle>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (mut pos, settings) in paddles {
        if input.pressed(settings.move_up) {
            pos.translation.y += 1.;
        }
        if input.pressed(settings.move_down) {
            pos.translation.y += -1.;
        }
    }
}
