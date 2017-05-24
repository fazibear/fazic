pub mod ast;
pub mod stack;
pub mod node_builder;
pub mod program;
pub mod execute;

pub mod parser {
    include!(concat!(env!("OUT_DIR"), "/parser.rs"));
}

pub fn step(fazic: &mut ::fazic::Fazic) {
    if fazic.program.running {
        execute::exec_node(fazic.program.current_node(), fazic);
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

    match parser::parse_all(&input) {
        Ok(ast::Entry(None, nodes)) => {
            println!("{:?}", nodes);
            execute::exec_each_node(nodes, fazic);
            if !fazic.program.running {
                fazic.text_buffer.prompt();
            }
        },
        Ok(ast::Entry(Some(line), nodes)) => {
             fazic.program.add_line(line as u16, nodes, input.clone());
        },
        Err(e) => {
            println!("Parse error!: {:?}", e);
            fazic.text_buffer.insert_line(&format!("{: >1$}", "^", e.column));
            fazic.text_buffer.insert_line("?SYNTAX ERROR");
            fazic.text_buffer.prompt();
        }
    };
}
