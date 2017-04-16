use fazic::runtime::ast::NodeElement;

pub struct Program {
    lines: Vec<(u16, Vec<NodeElement>)>
}

impl Program {
    pub fn new() -> Program {
        Program {
            lines: vec![],
        }
    }

}
