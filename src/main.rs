#![allow(unused)]

mod colors;

use std::string::ToString;
use bevy::prelude::*;
use bevy::window::WindowMode;
// use std::{ collections::btree_map::IterMut, io::{self, Write}, };
// mod tic_tac_toe;

fn main() {
    /*
    println!("Hello, Gamer!");

    'main_loop: loop {
        let mut input = Default::default();
        let games = ["Tic Tac Toe", "Rock Paper Scissors"];
        println!("These are the games we have: ");
        for (index, item) in games.iter().enumerate() {
            println!("\t{}. {}", index + 1, item);
        }
        print!("What game would you like to play: ");
        io::stdout().flush();
        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(e) => {
                println!("What the Fuck: {}", e);
                continue;
            }
        }

        println!("{:?} What the fuck", input);
        if input.trim() == "q" {
            break 'main_loop;
        }
        {
            let input: usize = match input.trim().parse() {
                Ok(ok) => ok,
                Err(err) => {
                    panic!("What the hell: {:?}", err);
                    continue 'main_loop;
                    0
                }
            };

            match input {
                1 => println!("tic_tact_toe goes here"),//tic_tac_toe::tic_tac_toe(),
                2 => println!("Rock Paper Scissors"),
                _ => println!("Invalid input"),
            }
        }
    }
    */
    App::new()
        .insert_resource(ClearColor(colors::CLEAR_COLOR))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "games".to_string(),
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

const GAMES: [&str; 2] = ["Tic Tac Toe", "Rock Paper Scissors"];
pub fn main_menu (mut commands: Commands) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(720.0, 720.0)),
            color: colors::BOARD,
            ..Default::default()
        },
        ..Default::default()
    });
}
