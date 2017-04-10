pub const CHARS: u16 = 1200;
pub const CHARS_PER_LINE: u16 = 40;
pub const LINES: u16 = 30;
pub const MAX_LINES: usize = 1000;
pub const MAX_LINE_CHARS: usize = 200;

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
    pub insert_mode: bool,
    pub line_offset: u16,
    pub additional_lines: u16,
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
            insert_mode: false,
            line_offset: 0,
            additional_lines: 0,
            lines: Vec::with_capacity(MAX_LINES),
        };
        buffer.lines = vec![
            Vec::with_capacity(MAX_LINE_CHARS),
            Vec::with_capacity(MAX_LINE_CHARS),
            Vec::with_capacity(MAX_LINE_CHARS),
            Vec::with_capacity(MAX_LINE_CHARS),
            Vec::with_capacity(MAX_LINE_CHARS),
        ];
        buffer.lines[1] = vec![
            (' ', 0),
            (' ', 0),
            (' ', 0),
            (' ', 0),
            (' ', 0),
            ('*', 0),
            (' ', 0),
            ('*', 1),
            (' ', 0),
            ('*', 2),
            (' ', 0),
            ('*', 3),
            (' ', 0),
            ('*', 4),
            (' ', 0),
            ('*', 5),
            (' ', 0),
            ('F', 14),
            ('A', 14),
            ('Z', 14),
            ('I', 14),
            ('C', 14),
            ('!', 14),
            (' ', 0),
            ('*', 5),
            (' ', 0),
            ('*', 4),
            (' ', 0),
            ('*', 3),
            (' ', 0),
            ('*', 2),
            (' ', 0),
            ('*', 1),
            (' ', 0),
            ('*', 0),
        ];
        buffer.lines[3] = vec![
            ('R', 14),
            ('E', 14),
            ('A', 14),
            ('D', 14),
            ('Y', 14),
            ('.', 14),
        ];
        buffer.update_chars();
        buffer.update_cursor();
        buffer
    }

    pub fn refreshed(&mut self) {
        self.changed = false;
    }

    pub fn update_cursor(&mut self) {
        let mut pos: u16 = 0;
        for line in self.line_offset..self.cursor_line  {
            pos = pos + self.lines[line as usize].len() as u16;
            pos = pos + CHARS_PER_LINE;
            pos = pos - pos % CHARS_PER_LINE;
        }
        pos = pos + self.cursor_char;
        if pos >= CHARS {
            pos = pos - self.cursor_char + self.cursor_char % CHARS_PER_LINE;
        }
        self.cursor = pos;
        self.changed = true;
    }

    pub fn update_chars(&mut self) {
        let mut pos = 0;

        let start = self.line_offset as usize;
        let len = self.lines.len() as usize;
        let end = if start + LINES as usize > len {
            len
        } else {
            start + LINES as usize
        };

        self.additional_lines = 0;
        for line in self.lines[start..end].iter() {
            self.additional_lines = self.additional_lines + line.len() as u16 / CHARS_PER_LINE;
            for &(char, color) in line {
                if pos >= CHARS { break; }
                self.chars[pos as usize] = char;
                self.colors[pos as usize] = color;
                if pos >= CHARS - 1 && self.cursor / CHARS_PER_LINE == LINES - 1 {
                    let mut chars = ['\0'; CHARS as usize];
                    let mut colors = [self.current_color; CHARS as usize];

                    for i in CHARS_PER_LINE..CHARS {
                        chars[i as usize - CHARS_PER_LINE as usize] = self.chars[i as usize];
                        colors[i as usize - CHARS_PER_LINE as usize] = self.colors[i as usize];
                    }
                    self.chars = chars;
                    self.colors = colors;
                    pos = pos - CHARS_PER_LINE;
                    self.line_offset = self.line_offset + 1;
                }
                pos = pos + 1;
            }
            if pos < CHARS {
                for _ in 0..CHARS_PER_LINE - pos % CHARS_PER_LINE {
                    self.chars[pos as usize] = ' ';
                    pos = pos + 1;
                }
            }
        }
        while pos < CHARS {
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
            if self.cursor_line < self.line_offset {
                self.line_offset = self.line_offset - 1;
                self.update_chars();
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
            if self.cursor_line > self.line_offset + LINES - 1 - self.additional_lines {
                self.line_offset = self.line_offset + 1;
                self.update_chars();
            }
            self.update_cursor();
        }
    }

    pub fn enter(&mut self) {
        if self.lines.len() - 1 == self.cursor_line as usize {
            self.lines.push(Vec::with_capacity(MAX_LINE_CHARS));
        }
        self.cursor_line = self.cursor_line + 1;
        if self.cursor_line > self.line_offset + LINES - 1 - self.additional_lines {
            self.line_offset = self.line_offset + 1;
        }
        self.cursor_char = 0;
        self.update_chars();
        self.update_cursor();
    }

    pub fn insert_string(&mut self, string: String) {
        for char in string.chars() {
            if self.insert_mode || self.cursor_char == self.lines[self.cursor_line as usize].len() as u16 {
                self.lines[self.cursor_line as usize].insert(self.cursor_char as usize, (char, self.current_color));
            } else {
                self.lines[self.cursor_line as usize][self.cursor_char as usize] = (char, self.current_color);
            }
            self.cursor_char = self.cursor_char + 1;
        }
        self.update_chars();
        self.update_cursor();
        self.changed = true;
    }

    pub fn backspace(&mut self) {
        if self.cursor_char != 0 {
            self.lines[self.cursor_line as usize].remove(self.cursor_char as usize - 1);
            self.changed = true;
            self.cursor_char = self.cursor_char - 1;
            self.update_chars();
            self.update_cursor();
        }
    }

    pub fn set_current_color(&mut self, color: u8) {
        self.current_color = color;
        self.changed = true;
    }
}
