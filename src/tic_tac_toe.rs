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




pub fn tic_tac_toe(mut commands: Commands) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::Oklcha(bevy::color::Oklcha {
                lightness: 0.6299,
                chroma: 0.14,
                hue: 318.22,
                alpha: 1.0,
            }),
            ..Default::default()
        },
        ..Default::default()
    });
}
