use config::*;

pub struct TextBuffer {
    pub chars: [char; TEXT_BUFFER_CHARS as usize],
    pub colors: [u8; TEXT_BUFFER_CHARS as usize],
    pub cursor: i32,
    pub cursor_line: i32,
    pub cursor_char: i32,
    pub current_color: u8,
    pub background_color: u8,
    pub changed: bool,
    pub show_cursor: bool,
    pub lines: Vec<Vec<(char, u8)>>,
    pub insert_mode: bool,
    pub line_offset: i32,
    pub additional_lines: i32,
}

impl Default for TextBuffer {
    fn default() -> TextBuffer {
        let mut buffer = TextBuffer {
            chars: [' '; TEXT_BUFFER_CHARS as usize],
            colors: [14; TEXT_BUFFER_CHARS as usize],
            current_color: 14,
            cursor: 0,
            cursor_line: 3,
            cursor_char: 0,
            background_color: 6,
            changed: true,
            show_cursor: true,
            insert_mode: false,
            line_offset: 0,
            additional_lines: 0,
            lines: Vec::with_capacity(TEXT_BUFFER_MAX_LINES as usize),
        };
        buffer.lines = vec![
            Vec::with_capacity(TEXT_BUFFER_MAX_LINE_CHARS as usize),
            Vec::with_capacity(TEXT_BUFFER_MAX_LINE_CHARS as usize),
            Vec::with_capacity(TEXT_BUFFER_MAX_LINE_CHARS as usize),
            Vec::with_capacity(TEXT_BUFFER_MAX_LINE_CHARS as usize),
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
        buffer.prompt();
        buffer.update_chars();
        buffer.update_cursor();
        buffer
    }
}

impl TextBuffer {
    pub fn new() -> TextBuffer {
        TextBuffer::default()
    }

    pub fn prompt(&mut self) {
        self.insert_line("READY.");
    }

    pub fn insert_line(&mut self, string: &str) {
        self.insert_string(string.to_string());
        self.enter();
    }

    pub fn refreshed(&mut self) {
        self.changed = false;
    }

    pub fn blink_cursor(&mut self) {
        self.show_cursor = !self.show_cursor;
        self.changed = true;
    }

    pub fn left(&mut self) {
        if self.cursor_char != 0 {
            self.cursor_char -= 1;
            self.update_cursor();
        }
    }

    pub fn right(&mut self) {
        if self.cursor_char != self.lines[self.cursor_line as usize].len() as i32 {
            self.cursor_char += 1;
            self.update_cursor();
        }
    }

    pub fn up(&mut self) {
        if self.cursor_line != 0 {
            self.cursor_line -= 1;
            if self.cursor_char as usize > self.lines[self.cursor_line as usize].len() {
                self.cursor_char = self.lines[self.cursor_line as usize].len() as i32;
            }
            if self.cursor_line < self.line_offset {
                self.line_offset -= 1;
                self.update_chars();
            }
            self.update_cursor();
        }
    }

    pub fn down(&mut self) {
        if self.cursor_line + 1 != self.lines.len() as i32 {
            self.cursor_line += 1;
            if self.cursor_char as usize > self.lines[self.cursor_line as usize].len() {
                self.cursor_char = self.lines[self.cursor_line as usize].len() as i32;
            }
            if self.cursor_line > self.line_offset + TEXT_BUFFER_LINES - 1 - self.additional_lines {
                self.line_offset += 1;
                self.update_chars();
            }
            self.update_cursor();
        }
    }

    pub fn enter(&mut self) {
        self.add_buffer_line();
        self.cursor_line += 1;
        if self.cursor_line > self.line_offset + TEXT_BUFFER_LINES - 1 - self.additional_lines {
            self.line_offset += 1;
        }
        self.cursor_char = 0;
        self.update_chars();
        self.update_cursor();
    }

    pub fn insert_string(&mut self, string: String) {
        for char in string.chars() {
            if self.insert_mode
                || self.cursor_char == self.lines[self.cursor_line as usize].len() as i32
            {
                self.lines[self.cursor_line as usize]
                    .insert(self.cursor_char as usize, (char, self.current_color));
            } else {
                self.lines[self.cursor_line as usize][self.cursor_char as usize] =
                    (char, self.current_color);
            }
            self.cursor_char += 1;
        }
        self.update_chars();
        self.update_cursor();
        self.changed = true;
    }

    pub fn backspace(&mut self) {
        if self.cursor_char != 0 {
            self.lines[self.cursor_line as usize].remove(self.cursor_char as usize - 1);
            self.changed = true;
            self.cursor_char -= 1;
            self.update_chars();
            self.update_cursor();
        }
    }

    pub fn set_current_color(&mut self, color: u8) {
        self.current_color = color;
        self.changed = true;
    }

    pub fn get_current_line_string(&mut self) -> String {
        self.lines[self.cursor_line as usize]
            .iter()
            .map(|&(c, _)| c)
            .collect()
    }

    /* private */

    fn update_cursor(&mut self) {
        let mut pos = 0;
        for line in self.line_offset..self.cursor_line {
            pos += self.lines[line as usize].len() as i32;
            pos += TEXT_BUFFER_CHARS_PER_LINE;
            pos -= pos % TEXT_BUFFER_CHARS_PER_LINE;
        }
        pos += self.cursor_char;
        if pos >= TEXT_BUFFER_CHARS {
            pos -= self.cursor_char + self.cursor_char % TEXT_BUFFER_CHARS_PER_LINE;
        }
        self.cursor = pos;
        self.changed = true;
    }

    fn update_chars(&mut self) {
        let mut pos = 0;

        let start = self.line_offset as usize;
        let len = self.lines.len() as usize;
        let end = if start + TEXT_BUFFER_LINES as usize > len {
            len
        } else {
            start + TEXT_BUFFER_LINES as usize
        };

        self.additional_lines = 0;
        for line in self.lines[start..end].iter() {
            self.additional_lines += line.len() as i32 / TEXT_BUFFER_CHARS_PER_LINE;
            for &(char, color) in line {
                if pos >= TEXT_BUFFER_CHARS {
                    break;
                }
                self.chars[pos as usize] = char;
                self.colors[pos as usize] = color;
                if pos >= TEXT_BUFFER_CHARS - 1
                    && self.cursor / TEXT_BUFFER_CHARS_PER_LINE == TEXT_BUFFER_LINES - 1
                {
                    let mut chars = ['\0'; TEXT_BUFFER_CHARS as usize];
                    let mut colors = [self.current_color; TEXT_BUFFER_CHARS as usize];

                    for i in TEXT_BUFFER_CHARS_PER_LINE..TEXT_BUFFER_CHARS {
                        chars[i as usize - TEXT_BUFFER_CHARS_PER_LINE as usize] =
                            self.chars[i as usize];
                        colors[i as usize - TEXT_BUFFER_CHARS_PER_LINE as usize] =
                            self.colors[i as usize];
                    }
                    self.chars = chars;
                    self.colors = colors;
                    pos -= TEXT_BUFFER_CHARS_PER_LINE;
                    self.line_offset += 1;
                }
                pos += 1;
            }
            if pos < TEXT_BUFFER_CHARS {
                for _ in 0..TEXT_BUFFER_CHARS_PER_LINE - pos % TEXT_BUFFER_CHARS_PER_LINE {
                    self.chars[pos as usize] = ' ';
                    pos += 1;
                }
            }
        }
        while pos < TEXT_BUFFER_CHARS {
            self.chars[pos as usize] = ' ';
            pos += 1;
        }
        self.changed = true;
    }

    fn add_buffer_line(&mut self) {
        if self.lines.len() - 1 == self.cursor_line as usize {
            if self.lines.len() == TEXT_BUFFER_MAX_LINES as usize {
                self.lines.remove(0);
                self.cursor_line -= 1;
            }
            self.lines
                .push(Vec::with_capacity(TEXT_BUFFER_MAX_LINE_CHARS as usize));
        }
    }
}
