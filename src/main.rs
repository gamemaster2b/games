#![allow(unused)]

mod colors;

use bevy::prelude::*;
//! use std::{ collections::btree_map::IterMut, io::{self, Write}, };
//! mod tic_tac_toe;

fn main() {
    /*
    println!("Hello, Gamer!");

    'main_loop: loop {
        let mut input = Default::default();
        let games = ["Tic Tac Toe", "Rock Paper Scissors"];
        println!("This are the games we have: ");
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

        println!("{:?} waad", input);
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
    App::new().add_plugins(DefailtPlugins) ;
}
