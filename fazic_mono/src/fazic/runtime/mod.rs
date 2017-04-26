extern crate lalrpop_util;

pub mod ast;
pub mod parser;
pub mod node_builder;
pub mod program;
pub mod execute;

use self::ast::{Entry};

pub fn step(fazic: &mut ::fazic::Fazic) {
    if fazic.program.running {
        execute::exec_node(fazic.program.current_node(), fazic);
        fazic.program.next();
        if !fazic.program.running {
            fazic.text_buffer.prompt();
        }
    }
}

pub fn exec(fazic: &mut ::fazic::Fazic) {
    let input = fazic.text_buffer.get_current_line_string();
    if input.len() == 0 {
        fazic.text_buffer.insert_line("");
        return;
    }
    fazic.text_buffer.enter();
    let ast = parser::parse_all(&input);

    match ast {
        Ok(Entry(None, nodes)) => {
            execute::exec_each_node(nodes, fazic);
            fazic.text_buffer.enter();
            if !fazic.program.running {
                fazic.text_buffer.prompt();
            }
        },
        Ok(Entry(line, ast)) => {
            fazic.program.add_line(line.unwrap() as u16, ast, input.clone());
        }
        Err(lalrpop_util::ParseError::InvalidToken{location}) => {
            fazic.text_buffer.insert_line(&format!("{: >1$}", "^", location + 1));
            fazic.text_buffer.insert_line("?SYNTAX ERROR");
            fazic.text_buffer.prompt();
        }
        _ => {
            fazic.text_buffer.enter();
            fazic.text_buffer.insert_line("?SYNTAX ERROR");
            fazic.text_buffer.prompt();
        }
    }
}
