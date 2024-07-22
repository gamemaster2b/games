//use std::io;

fn main() {
    println!("Hello, Gamer!");
    let games = ["Tic Tac Toe", "Rock Paper Scissors"];
    println!("This are the games we have: ");
    for index in 0..games.len() {
        println!("\t{}. {}", index + 1, games[index]);
    }
    print!("What game would you like to play: ");
}
