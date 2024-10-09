use itertools::Itertools;
use bevy::prelude::*;
use std::{
    collections::btree_map::IterMut,
    io::{self, Write},
};

#[derive(Component)]
struct Position {
    x: u8,
    y: u8,
}

impl Position {
    fn new(x: u8, y: u8) -> Position {
        Position { x, y }
    }
}

#[derive(Component)]
struct TileSymbol {
    symbol: String,
}

impl TileSymbol {
    fn new(symbol: &str) -> TileSymbol {
        TileSymbol {
            symbol: symbol.to_string(),
        }
    }
}

#[derive(Component)]
struct Board {
    size: u8,
    chain: u8,
}

impl Board {
    fn new(size: u8, chain: u8) -> Board {
        Board { size, chain }
    }

    fn print(&self) {
        for i in 0..self.size {
            for j in 0..self.size {
                print!("{} ", "-");
            }
            println!();
        }
    }
}
impl Default for Board {
    fn default() -> Self {
        Self {
            size: 3,
            chain: 3,
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

    print!("Chain length: ");
    io::stdout().flush();

    let mut board_chain = Default::default();
    match io::stdin().read_line(&mut board_chain) {
        Ok(_) => {}
        Err(e) => {
            println!("Chain Issue: {:?}", e)
        }
    }
    let board_chain: u8 = match board_chain.trim().parse() {
        Ok(chain) => chain,
        Err(e) => {
            println!("Chain issue: {:?}", e);
            return;
        }
    };
    
    let mut world: World = Default::default();
    
    world.spawn(Board::new(board_size, board_chain)).insert(|tiles: &mut World| {
        for tile in (0..board_size).cartesian_product(0..board_size) {
            tiles.spawn(Position::new(tile.0, tile.1)).insert(TileSymbol::new(" "));
        }
    });

    let board = Board::new(board_size, board_chain);
    println!("This is the board: ");
    board.print();

    println!("Enter your move in the form: (x,y) : ");
    io::stdout().flush();

    let mut play = Default::default();
    match io::stdin().read_line(&mut play) {
        Ok(_) => {}
        Err(e) => {
            println!("Play Fucked: {:?}", e)
        }
    }
    let mut play: Vec<&str> = play.trim().split(&[' ', ',', '(', ')'][..]).collect();

    println!("{:?}", play);
}
