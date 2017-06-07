use ::fazic::*;
use ::fazic::ast::*;
use ::fazic::execute::commands::*;

const TRUE: NodeElement = NodeElement::Value(Value::Bool(true));
const FALSE: NodeElement = NodeElement::Value(Value::Bool(false));
const INTEGER_1: NodeElement = NodeElement::Value(Value::Integer(1));
//const INTEGER_5: NodeElement = NodeElement::Value(Value::Integer(5));
//const test_string: NodeElement = NodeElement::Value(Value::String("TEST".to_string()));

#[test]
fn print_true() {
    let mut fazic = Fazic::new();

    print(&mut fazic, vec![TRUE]);
    fazic.text_buffer.cursor_line = fazic.text_buffer.cursor_line - 1;

    assert_eq!(
        fazic.text_buffer.get_current_line_string(),
        "true".to_string()
    )
}

#[test]
fn print_false() {
    let mut fazic = Fazic::new();

    print(&mut fazic, vec![FALSE]);
    fazic.text_buffer.cursor_line = fazic.text_buffer.cursor_line - 1;

    assert_eq!(
        fazic.text_buffer.get_current_line_string(),
        "false".to_string()
    )
}

#[test]
fn print_integer() {
    let mut fazic = Fazic::new();

    print(&mut fazic, vec![INTEGER_1]);
    fazic.text_buffer.cursor_line = fazic.text_buffer.cursor_line - 1;

    assert_eq!(
        fazic.text_buffer.get_current_line_string(),
        "1".to_string()
    )
}
