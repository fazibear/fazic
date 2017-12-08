use std::collections::BTreeMap;

pub struct Lines {
    map: BTreeMap<u16, usize>,
    current: u16,
}

impl Default for Lines {
    fn default() -> Lines {
        Lines {
            map: BTreeMap::new(),
            current: 0,
        }
    }
}

impl Lines {
    pub fn new() -> Lines {
        Lines::default()
    }

    pub fn reset(&mut self) {
        &self.map.clear();
        self.current = 0;
    }

    pub fn add(&mut self, line: u16, position: usize) {
        self.current = line;
        &self.map.insert(line, position);
    }

    pub fn current(&self) -> u16 {
        self.current
    }

    pub fn get(&self, line: &u16) -> Option<&usize> {
        self.map.get(line)
    }

    pub fn get_next(&self, line: &u16) -> Option<&usize> {
        match self.map.get(line) {
            Some(pos) => self.map.values().skip_while(|&x| x != pos).skip(1).next(),
            err => err,
        }
    }

    pub fn what_line(&self, pos: usize) -> u16 {
        let mut line: u16 = 0;

        for (key, val) in self.map.iter() {
            if *val >= pos {
                line = *key;
                break;
            }
        }

        line
    }
}
