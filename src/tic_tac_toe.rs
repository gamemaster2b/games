use itertools::Itertools;
use std::{
    collections::btree_map::IterMut,
    io::{self, Write},
};

struct Tile {
    value: char,
    x: u8,
    y: u8,
}

impl Tile {
    fn update(&mut self, value: &char) {
        self.value = *value;
    }
}
impl Default for Tile {
    fn default() -> Self {
        Self {
            value: ' ',
            x: 0,
            y: 0,
        }
    }
}

struct Board {
    size: u8,
    tiles: Vec<Tile>,
}
impl Board {
    fn new(size: u8) -> Board {
        let mut tiles: Vec<Tile> = Vec::new();
        for item in (0..size).cartesian_product(0..size) {
            tiles.push(Tile {
                x: item.0,
                y: item.1,
                ..Default::default()
            });
        }
        Board { size, tiles }
    }
}
impl Default for Board {
    fn default() -> Self {
        let mut d_tiles: Vec<Tile> = Vec::new();
        for item in (0..3).cartesian_product(0..3) {
            d_tiles.push(Tile {
                x: item.0,
                y: item.1,
                ..Default::default()
            });
        }
        Self {
            size: 3,
            tiles: d_tiles,
        }
    }
}
fn print(out: &str) {
    print!("{:?}", out);
    io::stdout().flush();
}

pub fn tic_tac_toe() {
    print("Would you like to vs the computer(c) or a friend(f): ");
    let mut input = Default::default();
    match io::stdin().read_line(&mut input) {
        Ok(_) => (),
        Err(e) => println!("That is not a valid input: {}", e),
    }
    print("What size board would you like to play on: ");
    let mut board_size = Default::default();
    match io::stdin().read_line(&mut board_size) {
        Ok(_) => (),
        Err(e) => println!("That is not a valid input: {}", e),
    }
    let board_size: u8 = match board_size.trim().parse() {
        Ok(ok) => ok,
        Err(err) => {
            println!("That is not a valid input: {:?}", err);
            return;
        }
    };
    let board = Board::new(board_size);
}
