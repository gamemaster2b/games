use bevy::scene::ron::de;
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
    chain: u8,
    tiles: Vec<Tile>,
}
impl Board {
    fn new(size: u8, chain: u8) -> Board {
        let mut tiles: Vec<Tile> = Vec::new();
        for item in (0..size).cartesian_product(0..size) {
            tiles.push(Tile {
                x: item.0,
                y: item.1,
                ..Default::default()
            });
        }
        Board { size, chain, tiles }
    }
    fn print(&self) {
        print!("y\\x ");
        for x in (0..self.size) {
                print!(" {} ", x);
            }
        println!();
        for y in (0..self.size) {
            print!("  {} ", y);
            for x in (0..self.size) {
                print!("|{}|", self.tiles[(y * self.size + x) as usize].value);
            }
            println!();
        }
    }
}
impl Default for Board {
    fn default() -> Self {
        let default_board = Board::new(3, 3);
        Self {
            size: default_board.size,
            chain: default_board.chain,
            tiles: default_board.tiles,
        }
    }
}

pub fn tic_tac_toe() {
    print!("Would you like to vs the computer(c) or a friend(f): ");
    io::stdout().flush();

    let mut input = Default::default();
    match io::stdin().read_line(&mut input) {
        Ok(_) => (),
        Err(e) => println!("That is not a valid input: {}", e),
    }


    print!("What size board would you like to play on: ");
    io::stdout().flush();

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

    print!("Chain length");
    io::stdout().flush();

    let mut board_chain = Default::default();
    match io::stdin().read_line(&mut board_chain) {
        Ok(_) => {}
        Err(e) => { println!("Chain Issuue: {:?}", e)}
    }
    let board_chain: u8 = match board_chain.trim().parse() {
        Ok(chain) => chain,
        Err(e) => { println!("Chain issue: {:?}", e); return;}
    };

    
    let board = Board::new(board_size, board_chain);
    println!("This is the board: ");
    board.print();
}
