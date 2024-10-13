use bevy::prelude::Color;

pub const BOARD: Color = Color::Oklcha(bevy::color::Oklcha {
    lightness: 0.191,
    chroma: 0.036,
    hue: 273.2,
    alpha: 1.0,
});

pub const TILE_PLACEHOLDER: Color = Color::Oklcha(bevy::color::Oklcha {
    lightness: 0.6299,
    chroma: 0.14,
    hue: 318.22,
    alpha: 1.0,
});

pub const TILE: Color = Color::Oklcha(bevy::color::Oklcha {
    lightness: 0.40,
    chroma: 0.14,
    hue: 325.06,
    alpha: 1.0,
});

pub const TEXT: Color = Color::Oklcha(bevy::color::Oklcha {
    lightness: 0.50,
    chroma: 0.14,
    hue: 330.06,
    alpha: 1.0,
});

pub const CLEAR_COLOR: Color = Color::Oklcha(bevy::color::Oklcha {
    lightness: 0.2706,
    chroma: 0.035,
    hue: 267.69,
    alpha: 1.0,
});
