use ::fazic::*;

#[test]
fn abs() {
    let mut fazic = Fazic::new();
    fazic.text_buffer.insert_string("? ABS(-2)".to_string());
    fazic.enter_key();
    for _ in 0..100 { fazic.tick(); }
    fazic.text_buffer.cursor_line = fazic.text_buffer.cursor_line - 2;

    assert_eq!(
        fazic.text_buffer.get_current_line_string(),
        "2".to_string()
    )
}

#[test]
fn sin() {
    let mut fazic = Fazic::new();
    fazic.text_buffer.insert_string("? SIN(1)".to_string());
    fazic.enter_key();
    for _ in 0..100 { fazic.tick(); }
    fazic.text_buffer.cursor_line = fazic.text_buffer.cursor_line - 2;

    assert_eq!(
        fazic.text_buffer.get_current_line_string(),
        "0.8415".to_string()
    )
}

#[test]
fn cos() {
    let mut fazic = Fazic::new();
    fazic.text_buffer.insert_string("? COS(1)".to_string());
    fazic.enter_key();
    for _ in 0..100 { fazic.tick(); }
    fazic.text_buffer.cursor_line = fazic.text_buffer.cursor_line - 2;

    assert_eq!(
        fazic.text_buffer.get_current_line_string(),
        "0.5403".to_string()
    )
}

#[test]
fn tan() {
    let mut fazic = Fazic::new();
    fazic.text_buffer.insert_string("? TAN(1)".to_string());
    fazic.enter_key();
    for _ in 0..100 { fazic.tick(); }
    fazic.text_buffer.cursor_line = fazic.text_buffer.cursor_line - 2;

    assert_eq!(
        fazic.text_buffer.get_current_line_string(),
        "1.5574".to_string()
    )
}