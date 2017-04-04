extern crate sdl2;

pub const BLACK:       sdl2::pixels::Color = sdl2::pixels::Color::RGB(  0,   0,   0);
pub const WHITE:       sdl2::pixels::Color = sdl2::pixels::Color::RGB(255, 255, 255);
pub const RED:         sdl2::pixels::Color = sdl2::pixels::Color::RGB(224,  64,  64);
pub const CYAN:        sdl2::pixels::Color = sdl2::pixels::Color::RGB( 96, 255, 255);
pub const MAGENTA:     sdl2::pixels::Color = sdl2::pixels::Color::RGB(224,  96, 224);
pub const GREEN:       sdl2::pixels::Color = sdl2::pixels::Color::RGB( 64, 224,  64);
pub const BLUE:        sdl2::pixels::Color = sdl2::pixels::Color::RGB( 64,  64, 224);
pub const YELLOW:      sdl2::pixels::Color = sdl2::pixels::Color::RGB(255, 255,  64);
pub const ORNAGE:      sdl2::pixels::Color = sdl2::pixels::Color::RGB(224, 160,  64);
pub const BROWN:       sdl2::pixels::Color = sdl2::pixels::Color::RGB(156, 116,  72);
pub const PINK:        sdl2::pixels::Color = sdl2::pixels::Color::RGB(255, 160, 160);
pub const DARK_GREY:   sdl2::pixels::Color = sdl2::pixels::Color::RGB( 84,  84,  84);
pub const GREY:        sdl2::pixels::Color = sdl2::pixels::Color::RGB(136, 136, 136);
pub const LIGHT_GREEN: sdl2::pixels::Color = sdl2::pixels::Color::RGB(160, 255, 160);
pub const LIGHT_BLUE:  sdl2::pixels::Color = sdl2::pixels::Color::RGB(160, 160, 255);
pub const LIGHT_GREY:  sdl2::pixels::Color = sdl2::pixels::Color::RGB(192, 192, 192);

pub enum Color {
    Black,
    White,
    Red,
    Cyan,
    Magenta,
    Green,
    Blue,
    Yellow,
    Orange,
    Brown,
    Pink,
    DarkGrey,
    Grey,
    LightGreen,
    LightBlue,
    LightGrey
}

impl Color {
    pub fn value(&self) -> sdl2::pixels::Color {
        match *self {
            Color::Black      => BLACK,
            Color::White      => WHITE,
            Color::Red        => RED,
            Color::Cyan       => CYAN,
            Color::Magenta    => MAGENTA,
            Color::Green      => GREEN,
            Color::Blue       => BLUE,
            Color::Yellow     => YELLOW,
            Color::Orange     => ORNAGE,
            Color::Brown      => BROWN,
            Color::Pink       => PINK,
            Color::DarkGrey   => DARK_GREY,
            Color::Grey       => GREY,
            Color::LightGreen => LIGHT_GREEN,
            Color::LightBlue  => LIGHT_BLUE,
            Color::LightGrey  => LIGHT_GREY
        }
    }
}
