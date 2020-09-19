use enums::*;

pub struct Program {
    pub lines: Vec<(u16, String, Vec<NodeElement>)>,
}

impl Default for Program {
    fn default() -> Program {
        Program { lines: vec![] }
    }
}

impl Program {
    pub fn new() -> Program {
        Program::default()
    }

    pub fn add_line(&mut self, line: u16, nodes: Vec<NodeElement>, string: &str) {
        self.lines.retain(|&(l, _, _)| l != line);
        self.lines.push((line, string.to_string(), nodes));
        self.lines.sort_by(|&(a, _, _), &(b, _, _)| a.cmp(&b));
    }

    pub fn remove_line(&mut self, line: u16) {
        self.lines.retain(|&(l, _, _)| l != line);
    }

    pub fn nodes(&mut self, nodes: &mut Vec<NodeElement>) {
        for &(line, _, ref node) in &self.lines {
            nodes.push(NodeElement::LineNo(line));
            nodes.extend(node.clone());
        }
    }
}
