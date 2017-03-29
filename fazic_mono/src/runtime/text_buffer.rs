pub struct TextBuffer {
    pub string: String,
    pub colors: [i32; 1040],
    pub cursor: usize,
}

impl TextBuffer {
    pub fn new() -> TextBuffer {
        TextBuffer {
            string: "                                                     **** FAZIC ****                                                    READY.".to_string(),
            colors: [1; 1040],
            cursor: 160,
        }
    }
}
