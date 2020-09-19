use fazic::*;

#[test]
fn print_integer() {
    let mut fazic = Fazic::new();
    fazic.text_buffer.insert_string("? 1".to_string());
    fazic.enter_key();
    for _ in 0..100 {
        fazic.tick();
    }
    fazic.text_buffer.cursor_line = fazic.text_buffer.cursor_line - 2;

    assert_eq!(fazic.text_buffer.get_current_line_string(), "1".to_string())
}

#[test]
fn print_string() {
    let mut fazic = Fazic::new();
    fazic.text_buffer.insert_string("? \"TEST\"".to_string());
    fazic.enter_key();
    for _ in 0..100 {
        fazic.tick();
    }
    fazic.text_buffer.cursor_line = fazic.text_buffer.cursor_line - 2;

    assert_eq!(
        fazic.text_buffer.get_current_line_string(),
        "TEST".to_string()
    )
}
