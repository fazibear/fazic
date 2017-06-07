pub mod operators;
pub mod commands;
pub mod functions;

use ::fazic::ast::*;

pub fn exec_each_node(nodes: Vec<NodeElement>, fazic: &mut ::fazic::Fazic) {
    for node in nodes {
        exec_node(node, fazic)
    }
}

pub fn exec_node(node: NodeElement, fazic: &mut ::fazic::Fazic) {
    match node {
        NodeElement::Node(Node(Opcode::Print, params)) => {
            let params = eval_each_node(params, fazic);
            commands::print(fazic, params)
        },
        NodeElement::Node(Node(Opcode::Let, params)) => {
            let params = eval_each_node(params, fazic);
            commands::let_(fazic, params)
        },
        NodeElement::Node(Node(Opcode::New, _)) => commands::new(fazic),
        NodeElement::Node(Node(Opcode::Clr, _)) => commands::clr(fazic),
        NodeElement::Node(Node(Opcode::Cont, _)) => commands::cont(fazic),
        NodeElement::Node(Node(Opcode::List, _)) => commands::list(fazic),
        NodeElement::Node(Node(Opcode::Run, _)) => commands::run(fazic),
        NodeElement::Node(Node(Opcode::End, _)) => commands::end(fazic),
        NodeElement::Node(Node(Opcode::Stop, _)) => commands::stop(fazic),
        NodeElement::Node(Node(Opcode::Rem, _)) => (),
        NodeElement::Node(Node(Opcode::Load, params)) => commands::load(fazic, params),
        NodeElement::Node(Node(Opcode::Save, params)) => commands::save(fazic, params),

        NodeElement::Node(Node(Opcode::Goto, params)) => commands::goto(fazic, params),
        NodeElement::Node(Node(Opcode::Gosub, params)) => commands::gosub(fazic, params),
        NodeElement::Node(Node(Opcode::Return, _)) => commands::return_(fazic),

        NodeElement::Node(Node(Opcode::For, params)) => {
            let params = eval_each_node(params, fazic);
            commands::for_(fazic, params)
        },
        NodeElement::Node(Node(Opcode::Next, params)) => commands::next(fazic, params),

        NodeElement::Node(Node(Opcode::If, params)) => {
            let params = eval_each_node(params, fazic);
            commands::if_(fazic, params);
        },

        NodeElement::Node(Node(Opcode::Color, params)) => {
            let params = eval_each_node(params, fazic);
            commands::color(fazic, params)

        },
        NodeElement::Node(Node(Opcode::Mode, params)) => commands::mode(fazic, params),
        NodeElement::Node(Node(Opcode::Flip, _)) => commands::flip(fazic),
        NodeElement::Node(Node(Opcode::Dot, params)) => {
            let params = eval_each_node(params, fazic);
            commands::dot(fazic, params);
        },

        NodeElement::Node(_) => (),
        NodeElement::Value(_) => (),
        NodeElement::Error(e) => println!("ERROR: {}", e),
    }
}

pub fn eval_each_node(nodes: Vec<NodeElement>, fazic: &mut ::fazic::Fazic) -> Vec<NodeElement> {
    nodes
        .into_iter()
        .map(|node| eval_node(node, fazic))
        .collect()
}

pub fn eval_node(node: NodeElement, fazic: &mut ::fazic::Fazic) -> NodeElement {
    return match node {
        NodeElement::Node(Node(Opcode::Neg, params)) => operators::neg(eval_each_node(params, fazic)),

        NodeElement::Node(Node(Opcode::Var, params)) => var(params, fazic),
        NodeElement::Node(Node(Opcode::Add, params)) => operators::add(eval_each_node(params, fazic)),
        NodeElement::Node(Node(Opcode::Sub, params)) => operators::sub(eval_each_node(params, fazic)),
        NodeElement::Node(Node(Opcode::Mul, params)) => operators::mul(eval_each_node(params, fazic)),
        NodeElement::Node(Node(Opcode::Div, params)) => operators::div(eval_each_node(params, fazic)),
        NodeElement::Node(Node(Opcode::Eql, params)) => operators::eql(eval_each_node(params, fazic)),
        NodeElement::Node(Node(Opcode::NotEql, params)) => operators::not_eql(eval_each_node(params, fazic)),
        NodeElement::Node(Node(Opcode::Lt, params)) => operators::lt(eval_each_node(params, fazic)),
        NodeElement::Node(Node(Opcode::Gt, params)) => operators::gt(eval_each_node(params, fazic)),
        NodeElement::Node(Node(Opcode::LtEql, params)) => operators::lt_eql(eval_each_node(params, fazic)),
        NodeElement::Node(Node(Opcode::GtEql, params)) => operators::gt_eql(eval_each_node(params, fazic)),
        NodeElement::Node(Node(Opcode::Pow, params)) => operators::pow(eval_each_node(params, fazic)),

        NodeElement::Node(Node(Opcode::Not, params)) => operators::not(eval_each_node(params, fazic)),
        NodeElement::Node(Node(Opcode::And, params)) => operators::and(eval_each_node(params, fazic)),
        NodeElement::Node(Node(Opcode::Or, params)) => operators::or(eval_each_node(params, fazic)),

        NodeElement::Node(Node(Opcode::Abs, params)) => functions::abs(eval_each_node(params, fazic)),
        NodeElement::Node(Node(Opcode::Rnd, params)) => functions::rnd(eval_each_node(params, fazic)),

        NodeElement::Value(_) => node,
        _ => NodeElement::Error("Not implemented".to_string()),
    };
}

fn var(nodes: Vec<NodeElement>, fazic: &mut ::fazic::Fazic) -> NodeElement {
    let name = match nodes[0] {
        NodeElement::Value(Value::String(ref name)) => name.to_string(),
        _ => unreachable!(),
    };

    match fazic.program.variables.get(&name) {
        Some(value) => value.clone(),
        None => NodeElement::Value(Value::Integer(0))
    }
}
