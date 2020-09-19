use enums::Value;
use std::collections::HashMap;

pub struct Variables {
    variables: Vec<Value>,
    map: HashMap<String, usize>,
    counter: usize,
    empty: Value,
}

impl Default for Variables {
    fn default() -> Variables {
        Variables {
            variables: vec![],
            map: HashMap::with_capacity(10),
            counter: 0,
            empty: Value::Null,
        }
    }
}

impl Variables {
    pub fn new() -> Variables {
        Variables::default()
    }

    pub fn get(&self, i: usize) -> &Value {
        if i <= self.counter {
            &self.variables[i]
        } else {
            &self.empty
        }
    }

    pub fn set(&mut self, i: usize, val: Value) {
        if i <= self.counter {
            self.variables[i] = val
        }
    }

    pub fn alloc(&mut self, name: &str) -> usize {
        if let Some(i) = self.map.get(name) {
            return *i;
        }

        let i = self.counter;

        debug!("alocating {}, {}", name, i);
        self.map.insert(name.to_string(), i);
        self.variables.push(Value::Null);
        self.counter += 1;
        i
    }
}
