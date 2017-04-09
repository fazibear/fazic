pub const CHARS: u16 = 1000;

pub struct TextBuffer {
    pub chars: [char; CHARS as usize],
    pub colors: [u8; CHARS as usize],
    pub cursor: u16,
    pub cursor_line: u16,
    pub cursor_char: u16,
    pub current_color: u8,
    pub background_color: u8,
    pub changed: bool,
    pub show_cursor: bool,
    pub lines: Vec<Vec<(char, u8)>>,
}

impl TextBuffer {
    pub fn new() -> TextBuffer {
        let mut buffer = TextBuffer {
            chars: [' '; CHARS as usize],
            colors: [14; CHARS as usize],
            current_color: 14,
            cursor: 0,
            cursor_line: 4,
            cursor_char: 0,
            background_color: 6,
            changed: true,
            show_cursor: true,
            lines: vec![
                vec![],
                vec![
                    ('*', 0),
                    ('*', 1),
                    ('*', 2),
                    ('*', 3),
                    ('*', 4),
                    ('*', 5),
                    (' ', 0),
                    ('F', 14),
                    ('A', 14),
                    ('Z', 14),
                    ('I', 14),
                    ('C', 14),
                    ('!', 14),
                ],
                vec![],
                vec![
                    ('R', 14),
                    ('E', 14),
                    ('A', 14),
                    ('D', 14),
                    ('Y', 14),
                    ('.', 14),
                ],
                vec![],
            ],
        };
        buffer.update_chars();
        buffer.update_cursor();
        buffer
    }

    pub fn refreshed(&mut self) {
        self.changed = false;
    }

    pub fn update_cursor(&mut self) {
        let mut pos: u16 = 0;
        for line in 0..self.cursor_line {
            pos = pos + self.lines[line as usize].len() as u16;
            pos = pos + 40;
            pos = pos - pos % 40;
        }
        self.cursor = pos + self.cursor_char;
        self.changed = true;
    }

    pub fn update_chars(&mut self) {
        let mut pos = 0;
        for line in self.lines.iter() {
            for &(char, color) in line {
                self.chars[pos as usize] = char;
                self.colors[pos as usize] = color;
                pos = pos + 1;
            }
            for _ in 0..40 - pos % 40 {
                self.chars[pos as usize] = ' ';
                pos = pos + 1;
            }
        }
        while pos < 1000 {
            self.chars[pos as usize] = ' ';
            pos = pos + 1;
        }
        self.changed = true;
    }

    pub fn blink_cursor(&mut self) {
        self.show_cursor = !self.show_cursor;
        self.changed = true;
    }

    pub fn left(&mut self) {
        if self.cursor_char != 0 {
            self.cursor_char = self.cursor_char - 1;
            self.update_cursor();
        }
    }

    pub fn right(&mut self) {
        if self.cursor_char != self.lines[self.cursor_line as usize].len() as u16 {
            self.cursor_char = self.cursor_char + 1;
            self.update_cursor();
        }
    }

    pub fn up(&mut self) {
        if self.cursor_line != 0 {
            self.cursor_line = self.cursor_line - 1;
            if self.cursor_char as usize > self.lines[self.cursor_line as usize].len() {
                self.cursor_char = self.lines[self.cursor_line as usize].len() as u16;
            }
            self.update_cursor();
        }
    }

    pub fn down(&mut self) {
        if self.cursor_line + 1 != self.lines.len() as u16 {
            self.cursor_line = self.cursor_line + 1;
            if self.cursor_char as usize > self.lines[self.cursor_line as usize].len() {
                self.cursor_char = self.lines[self.cursor_line as usize].len() as u16;
            }
            self.update_cursor();
        }

    }

    pub fn enter(&mut self) {
        println!("{} == {}", self.lines.len(), self.cursor_line);
        if self.lines.len() - 1 == self.cursor_line as usize {
            println!("add");
            self.lines.push(vec![]);
        }
        self.cursor_line = self.cursor_line + 1;
        self.cursor_char = 0;
        self.update_chars();
        self.update_cursor();
    }

    pub fn insert_string(&mut self, string: String) {
        for char in string.chars() {
            self.lines[self.cursor_line as usize].insert(self.cursor_char as usize, (char, self.current_color));
            self.changed = true;
            self.cursor_char = self.cursor_char + 1;
        }
        self.update_chars();
        self.update_cursor();
    }

    pub fn backspace(&mut self) {
        if self.cursor_char != 0 {
            self.lines[self.cursor_line as usize].remove(self.cursor_char as usize - 1);
            self.changed = true;
            self.cursor_char = self.cursor_char - 1;
            self.update_chars();
            self.update_cursor();
            println!("{:?}", self.lines);
        }
    }

    pub fn set_current_color(&mut self, color: u8) {
        self.current_color = color;
        self.changed = true;
    }

    fn shift(&mut self) {
        // let mut chars = ['\0'; CHARS as usize];
        // let mut colors = [self.current_color; CHARS as usize];
        //
        // for i in 40..1000 {
        //     chars[i - 40] = self.chars[i];
        //     colors[i - 40] = self.colors[i];
        // }
        // self.chars = chars;
        // self.colors = colors;
    }
}
