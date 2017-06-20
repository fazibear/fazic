use fazic::ast::*;
use fazic::config::*;

pub struct Program {
    pub lines: Vec<(u16, String)>,
    pub ast: Vec<Vec<NodeElement>>,
}

impl Program {
    pub fn new() -> Program {
        Program {
            lines: vec![],
            ast: vec![vec![]; BASIC_MAX_LINES as usize],
        }
    }

    pub fn add_line(&mut self, line: u16, nodes: Vec<NodeElement>, string: String) {
        self.lines.retain(|&(l, _)| l != line);
        self.lines.push((line, string));
        self.lines.sort_by(|&(a, _), &(b, _)| a.cmp(&b));
        self.ast[line as usize] = nodes;
    }
}
