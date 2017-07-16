pub const SCREEN_WIDTH: u16 = 320;
pub const SCREEN_HEIGHT: u16 = 240;
pub const SCREEN_PIXELS: usize = SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize;
pub const SCREEN_RGB_PIXELS: usize = SCREEN_PIXELS * 3;

pub const TEXT_BUFFER_CHARS_PER_LINE: u16 = SCREEN_WIDTH / 8;
pub const TEXT_BUFFER_LINES: u16 = SCREEN_HEIGHT / 8;
pub const TEXT_BUFFER_CHARS: u16 = TEXT_BUFFER_CHARS_PER_LINE * TEXT_BUFFER_LINES;

pub const TEXT_BUFFER_MAX_LINES: u16 = TEXT_BUFFER_LINES * 1000;
pub const TEXT_BUFFER_MAX_LINE_CHARS: u16 = TEXT_BUFFER_CHARS_PER_LINE * 5;

//pub const BASIC_MAX_LINES: u16 = 10000;

//pub const TEXT_MODE: u8 = 0;
//pub const FLIP_MODE: u8 = 1;
//pub const INST_MODE: u8 = 2;

pub const HOST: &str = "http://localhost:8080";
