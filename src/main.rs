#![allow(unused)]

mod colors;

use std::string::ToString;
use bevy::{prelude::*,window::WindowMode};

// use std::{ collections::btree_map::IterMut, io::{self, Write}, };
// mod tic_tac_toe;

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
        .add_systems(Startup, ( spawn_camera, main_menu ))
        .run();
}

pub fn spawn_camera (mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

