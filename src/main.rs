#![allow(unused)]

use std::{io, ops::Index};
mod tic_tac_toe;

fn main() {
    println!("Hello, Gamer!");
    let mut input = Default::default();
    'main_loop: loop {
        let games = ["Tic Tac Toe", "Rock Paper Scissors"];
        println!("This are the games we have: ");
        for index in 0..games.len() {
            println!("\t{}. {}", index + 1, games[index]);
        }
        print!("What game would you like to play: ");
        match io::stdin().read_line(&mut input) {
            Ok(_) => println!("Let's play {:?}.", games[input.trim().parse::<usize>().unwrap_or("bye") - 1 ]  ),
            Err(_) => println!("What the fuck"),
        }
        if input == "q" {
            break 'main_loop;
        }
    }
}
