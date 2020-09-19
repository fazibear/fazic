use parser::parse_all;

#[test]
fn expression() {
    assert!(parse_all("? 2+(5+6)*5").is_ok())
}

#[test]
fn expression_with_funcition() {
    assert!(parse_all("? ABS(5+5-3*(3*3))").is_ok())
}

#[test]
fn exression_with_or() {
    assert!(parse_all("? 5 OR (5+5*(ABS(3)*3))").is_ok())
}

#[test]
fn command_print_with_expression() {
    assert!(parse_all("PRINT (5+5*(ABS(-3)*3))").is_ok())
}

#[test]
fn command_print2_with_expression() {
    assert!(parse_all("? (5+5*(ABS(3)*3))").is_ok())
}

#[test]
fn command_print_with_string() {
    assert!(parse_all("? 'pojedynczy sdfsdf #$%ERG łłdf'").is_ok())
}

#[test]
fn command_print2_with_string() {
    assert!(parse_all("PRINT\"sdfsjfg difug oi$#@%%GD DF\"").is_ok())
}

#[test]
fn print_single_quite() {
    assert!(parse_all("PRINT '123'").is_ok())
}

#[test]
fn print_single_quite_with_spaces() {
    assert!(parse_all("  PRINT '123'").is_ok())
}

#[test]
fn print_expression() {
    assert!(parse_all("PRINT 10+10-2*67/2").is_ok())
}

#[test]
fn print_float() {
    assert!(parse_all("PRINT 2.2*2").is_ok())
}

#[test]
fn print_div() {
    assert!(parse_all("PRINT 4/3").is_ok())
}

#[test]
fn div_by_zer() {
    assert!(parse_all("PRINT 4/0").is_ok())
}

#[test]
fn print_with_line() {
    assert!(parse_all("10 PRINT 4/3").is_ok())
}

#[test]
fn mutliple_print() {
    assert!(parse_all("PRINT 4/3:PRINT 2/1").is_ok())
}

#[test]
fn multiple_print_with_line() {
    assert!(parse_all("10 PRINT 4/3:PRINT 2/2").is_ok())
}
#[test]
fn multiple_print_with_line_downcase() {
    assert!(parse_all("10 PRINT 4/3:print 2/2").is_ok())
}
