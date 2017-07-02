use fazic::ast::*;
use fazic::stack::*;
use std::collections::HashMap;
use fazic::config::*;

use std::time::Instant;

pub struct Program {
    pub lines: Vec<(u16, String)>,
    pub ast: Vec<Vec<NodeElement>>,
    pub running: bool,
    pub position: (u16, u16),
    pub first_line: u16,
    pub last_line: u16,
    pub variables: HashMap<String, NodeElement>,
    pub stack: Vec<StackEntry>,
    pub instant: Instant,
}

impl Program {
    pub fn new() -> Program {
        Program {
            lines: vec![],
            ast: vec![vec![]; BASIC_MAX_LINES as usize],
            running: false,
            position: (0, 0),
            first_line: BASIC_MAX_LINES - 1,
            last_line: 0,
            variables: HashMap::new(),
            stack: vec![],
            instant: Instant::now(),
        }
    }

    pub fn add_line(&mut self, line: u16, nodes: Vec<NodeElement>, string: String) {
        self.add_to_ast(line, nodes);
        self.add_to_lines(line, string);
        self.adjust_lines(line);
    }

    pub fn reset(&mut self) {
        if self.last_line != 0 {
            self.position = (self.first_line, 0);
        }
    }

    pub fn start(&mut self) {
        if self.last_line != 0 {
            self.running = true;
            self.instant = Instant::now();
        }
    }

    pub fn stop(&mut self) {
        self.running = false;
        println!("{:?}", self.instant.elapsed());
    }

    pub fn current_node(&mut self) -> NodeElement {
        self.ast[self.position.0 as usize][self.position.1 as usize].clone()
    }

    pub fn next(&mut self) -> bool {
        let length = self.ast[self.position.0 as usize].len();

        if length == 0 {
            return false;
        }

        if self.position.1 as usize == length - 1 {
            self.position.1 = 0;
            loop {
                self.position.0 = self.position.0 + 1;
                if self.position.0 > self.last_line {
                    self.running = false;
                    self.position = (0,0);
                    return false;
                }
                if self.ast[self.position.0 as usize].len() != 0 {
                    break;
                }
            };
        } else {
            self.position.1 = self.position.1 + 1;
        }
        return true;
    }

    /* private */

    fn add_to_ast(&mut self, line: u16, nodes: Vec<NodeElement>) {
        // check vector boundries
        self.ast[line as usize] = nodes;
    }

    fn add_to_lines(&mut self, line: u16, string: String) {
        self.lines.retain(|&(l, _)| l != line);
        self.lines.push((line, string));
        self.lines.sort_by(|&(a, _), &(b, _)| a.cmp(&b));
    }

    fn adjust_lines(&mut self, line: u16) {
        if line < self.first_line {
            self.first_line = line;
        }
        if line > self.last_line {
            self.last_line = line;
        }
    }
}