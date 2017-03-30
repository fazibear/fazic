pub struct TextBuffer {
    pub string: String,
    pub colors: [i32; 1040],
    pub cursor: usize,
}

impl TextBuffer {
    pub fn new() -> TextBuffer {
        TextBuffer {
            string: "                                                     **** FAZIC ****                                                    READY.".to_string(),
            colors: [5; 1040],
            cursor: 160,
        }
    }

    pub fn left(&mut self) {
        self.cursor -= 1
    }

    pub fn right(&mut self) {
        self.cursor += 1
    }

    pub fn up(&mut self) {
        self.cursor -= 40;
        if self.cursor <= 0 {
            self.cursor = 0;
        }
    }

    pub fn down(&mut self) {
        self.cursor += 40;
        if self.cursor > 1040 {
            self.cursor = 1040;
        }
    }
}
