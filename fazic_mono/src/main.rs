pub mod parser;
pub mod ast;
pub mod node_builder;
pub mod runtime;

fn main() {
    // println!(
    //     "{:?}",
    //     parser::parse_all("(5*(ABS(3)*3))").unwrap()
    // );
    // println!(
    //     "{:?}",
    //     parser::parse_all("PRINT '123'").unwrap()
    // );
    //
    runtime::run(
        parser::parse_all("PRINT 10+10-2*67/2").unwrap()
    );

    runtime::run(
        parser::parse_all("PRINT 2.2*2").unwrap()
    );

    runtime::run(
        parser::parse_all("PRINT 4/3").unwrap()
    );
    runtime::run(
        parser::parse_all("PRINT 4/0").unwrap()
    );

}

#[test]
fn expression() {
    assert!(parser::parse_all("2+(5+6)*5").is_ok())
}

#[test]
fn expression_with_funcition() {
    assert!(parser::parse_all("ABS(5+5*(3*3))").is_ok())
}

#[test]
fn exression_with_or() {
    assert!(parser::parse_all("5 OR (5+5*(ABS(3)*3))").is_ok())
}

#[test]
fn command_print_with_expression() {
    assert!(parser::parse_all("PRINT (5+5*(ABS(3)*3))").is_ok())
}

#[test]
fn command_print2_with_expression() {
    assert!(parser::parse_all("? (5+5*(ABS(3)*3))").is_ok())
}

#[test]
fn command_print_with_string() {
    assert!(parser::parse_all("? 'pojedynczy sdfsdf #$%ERG łłdf'").is_ok())
}

#[test]
fn command_print2_with_string() {
    assert!(parser::parse_all("PRINT\"sdfsjfg difug oi$#@%%GD DF\"").is_ok())
}
