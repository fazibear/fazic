use fazic::runtime::ast::NodeElement;

pub struct Program {
    pub lines: Vec<(u16, Vec<NodeElement>, String)>
}

impl Program {
    pub fn new() -> Program {
        Program {
            lines: vec![],
        }
    }

    pub fn add_line(&mut self, line: u16, nodes: Vec<NodeElement>, string: String) {
        self.lines.retain(|&(l, _, _)| l != line);
        self.lines.push((line, nodes, string));
        self.sort();
    }

    fn sort(&mut self) {
        self.lines.sort_by(|&(a, _, _), &(b, _, _)| a.cmp(&b));
    }
}
