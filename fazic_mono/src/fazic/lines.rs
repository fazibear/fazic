use std::collections::HashMap;

pub struct Lines {
    map: HashMap<u16, usize>,
}

impl Default for Lines {
    fn default() -> Lines {
        Lines {
            map: HashMap::new(),
        }
    }
}

impl Lines {
    pub fn new() -> Lines {
        Lines::default()
    }

    pub fn reset(&mut self) {
        self.map = HashMap::new()
    }

    pub fn add(&mut self, line: u16, position: usize) {
        &self.map.insert(line, position);
    }

    pub fn get(&self, line: &u16) -> Option<&usize> {
        self.map.get(line)
    }

    pub fn what_line(&self, pos: usize) -> u16 {
        let mut line: u16 = 0;

        for (key, val) in self.map.iter() {
            if *val >= pos {
                line = *key;
            }
            println!("key: {} val: {}", key, val);
        }

        line
    }
}
