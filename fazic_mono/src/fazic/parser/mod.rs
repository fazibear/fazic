mod grammar;
mod node_builder;

use fazic::ast::{NodeElement};

pub fn parse(str: &str) -> Result<NodeElement, String> {
    let ast = grammar::parse_all(str);
    if ast.is_ok(){
        Ok(ast.unwrap())
    } else {
        Err("Error".to_string())
    }
}

#[test]
fn expression() {
    assert!(parse("2+(5+6)*5").is_ok())
}

#[test]
fn expression_with_funcition() {
    assert!(parse("ABS(5+5*(3*3))").is_ok())
}

#[test]
fn exression_with_or() {
    assert!(parse("5 OR (5+5*(ABS(3)*3))").is_ok())
}

#[test]
fn command_print_with_expression() {
    assert!(parse("PRINT (5+5*(ABS(3)*3))").is_ok())
}

#[test]
fn command_print2_with_expression() {
    assert!(parse("? (5+5*(ABS(3)*3))").is_ok())
}

#[test]
fn command_print_with_string() {
    assert!(parse("? 'pojedynczy sdfsdf #$%ERG łłdf'").is_ok())
}

#[test]
fn command_print2_with_string() {
    assert!(parse("PRINT\"sdfsjfg difug oi$#@%%GD DF\"").is_ok())
}

#[test]
fn print_single_quite() {
    assert!(parse("PRINT '123'").is_ok())
}

#[test]
fn print_expression(){
    assert!(parse("PRINT 10+10-2*67/2").is_ok())
}

#[test]
fn print_float(){
    assert!(parse("PRINT 2.2*2").is_ok())
}

#[test]
fn print_div(){
    assert!(parse("PRINT 4/3").is_ok())
}

#[test]
fn div_by_zer(){
    assert!(parse("PRINT 4/0").is_ok())
}
