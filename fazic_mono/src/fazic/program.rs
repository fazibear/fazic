use fazic::ast::*;

pub struct Program {
    pub lines: Vec<(u16, String, Vec<NodeElement>)>,
}

impl Default for Program {
    fn default() -> Program {
        Program {
            lines: vec![],
        }
    }
}

impl Program {
    pub fn new() -> Program {
        Program::default()
    }

    pub fn add_line(&mut self, line: u16, nodes: Vec<NodeElement>, string: String) {
        self.lines.retain(|&(l, _, _)| l != line);
        self.lines.push((line, string, nodes));
        self.lines.sort_by(|&(a, _, _), &(b, _, _),| a.cmp(&b));
    }
}
