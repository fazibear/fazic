pub struct TextBuffer {
    pub string: String,
    pub colors: [i32; 1000],
    pub cursor: usize,
}

impl TextBuffer {
    pub fn new() -> TextBuffer {
        TextBuffer {
            string: "                                                     **** FAZIC ****                                                    READY.".to_string(),
            colors: [5; 1000],
            cursor: 160,
        }
    }

    pub fn left(&mut self) {
        if self.cursor.checked_sub(1).is_some() {
            self.cursor -= 1;
        }
    }

    pub fn right(&mut self) {
        if self.cursor + 1 <= 999 {
            self.cursor += 1;
        } else {
            self.cursor -= 39;
            self.shift();
        }
    }

    pub fn up(&mut self) {
        if self.cursor.checked_sub(40).is_some() {
            self.cursor -= 40;
        }
    }

    pub fn down(&mut self) {
        if self.cursor + 40 < 999 {
            self.cursor += 40;
        } else {
            self.shift();
        }
    }

    pub fn shift(&mut self) {
        if self.string.len() > 40 {
            self.string = self.string[40..self.string.len()].to_string();
        } else {
            self.string = "".to_string();
        }
    }
}
