use fazic::runtime::ast::NodeElement;

pub struct Program {
    pub lines: Vec<(u16, String)>,
    pub ast: Vec<Vec<NodeElement>>,
    pub running: bool,
}

impl Program {
    pub fn new() -> Program {
        Program {
            lines: vec![],
            ast: vec![vec![]; ::fazic::BASIC_MAX_LINES as usize],
            running: false,
        }
    }

    pub fn add_line(&mut self, line: u16, nodes: Vec<NodeElement>, string: String) {
        self.add_to_ast(line, nodes);
        self.add_to_lines(line, string);
    }

    pub fn start(&mut self) {
        self.running = true;
    }

    pub fn stop(&mut self) {
        self.running = false;
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
}
