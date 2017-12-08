pub const BLACK: (u8, u8, u8) = (0, 0, 0);
pub const WHITE: (u8, u8, u8) = (255, 255, 255);
pub const RED: (u8, u8, u8) = (224, 64, 64);
pub const CYAN: (u8, u8, u8) = (96, 255, 255);
pub const MAGENTA: (u8, u8, u8) = (224, 96, 224);
pub const GREEN: (u8, u8, u8) = (64, 224, 64);
pub const BLUE: (u8, u8, u8) = (64, 64, 224);
pub const YELLOW: (u8, u8, u8) = (255, 255, 64);
pub const ORNAGE: (u8, u8, u8) = (224, 160, 64);
pub const BROWN: (u8, u8, u8) = (156, 116, 72);
pub const PINK: (u8, u8, u8) = (255, 160, 160);
pub const DARK_GREY: (u8, u8, u8) = (84, 84, 84);
pub const GREY: (u8, u8, u8) = (136, 136, 136);
pub const LIGHT_GREEN: (u8, u8, u8) = (160, 255, 160);
pub const LIGHT_BLUE: (u8, u8, u8) = (160, 160, 255);
pub const LIGHT_GREY: (u8, u8, u8) = (192, 192, 192);

// pub const BLACK:       (u8, u8, u8) = (  0,   0,   0);
// pub const WHITE:       (u8, u8, u8) = (255, 255, 255);
// pub const RED:         (u8, u8, u8) = (136,   0,   0);
// pub const CYAN:        (u8, u8, u8) = (170, 255, 238);
// pub const MAGENTA:     (u8, u8, u8) = (204,  68, 204);
// pub const GREEN:       (u8, u8, u8) = (  0, 204,  85);
// pub const BLUE:        (u8, u8, u8) = (  0,   0, 170);
// pub const YELLOW:      (u8, u8, u8) = (238, 238, 119);
// pub const ORNAGE:      (u8, u8, u8) = (221, 136,  85);
// pub const BROWN:       (u8, u8, u8) = (102,  68,   0);
// pub const PINK:        (u8, u8, u8) = (255, 119, 119);
// pub const DARK_GREY:   (u8, u8, u8) = ( 51,  51,  51);
// pub const GREY:        (u8, u8, u8) = (119, 119, 119);
// pub const LIGHT_GREEN: (u8, u8, u8) = (170, 255, 102);
// pub const LIGHT_BLUE:  (u8, u8, u8) = (  0, 136, 255);
// pub const LIGHT_GREY:  (u8, u8, u8) = (187, 187, 187);

// pub enum Color {
//     Black,
//     White,
//     Red,
//     Cyan,
//     Magenta,
//     Green,
//     Blue,
//     Yellow,
//     Orange,
//     Brown,
//     Pink,
//     DarkGrey,
//     Grey,
//     LightGreen,
//     LightBlue,
//     LightGrey,
// }
//
// impl Color {
//     pub fn value(&self) -> (u8, u8, u8) {
//         match *self {
//             Color::Black => BLACK,
//             Color::White => WHITE,
//             Color::Red => RED,
//             Color::Cyan => CYAN,
//             Color::Magenta => MAGENTA,
//             Color::Green => GREEN,
//             Color::Blue => BLUE,
//             Color::Yellow => YELLOW,
//             Color::Orange => ORNAGE,
//             Color::Brown => BROWN,
//             Color::Pink => PINK,
//             Color::DarkGrey => DARK_GREY,
//             Color::Grey => GREY,
//             Color::LightGreen => LIGHT_GREEN,
//             Color::LightBlue => LIGHT_BLUE,
//             Color::LightGrey => LIGHT_GREY,
//         }
//     }
//
//     pub fn index(&self) -> u8 {
//         match *self {
//             Color::Black => 0,
//             Color::White => 1,
//             Color::Red => 2,
//             Color::Cyan => 3,
//             Color::Magenta => 4,
//             Color::Green => 5,
//             Color::Blue => 6,
//             Color::Yellow => 7,
//             Color::Orange => 8,
//             Color::Brown => 9,
//             Color::Pink => 10,
//             Color::DarkGrey => 11,
//             Color::Grey => 12,
//             Color::LightGreen => 13,
//             Color::LightBlue => 14,
//             Color::LightGrey => 15,
//         }
//     }
// }
//
pub fn rgb_for(color: u8) -> (u8, u8, u8) {
    match color {
        1 => WHITE,
        2 => RED,
        3 => CYAN,
        4 => MAGENTA,
        5 => GREEN,
        6 => BLUE,
        7 => YELLOW,
        8 => ORNAGE,
        9 => BROWN,
        10 => PINK,
        11 => DARK_GREY,
        12 => GREY,
        13 => LIGHT_GREEN,
        14 => LIGHT_BLUE,
        15 => LIGHT_GREY,
        _ => BLACK,
    }
}
