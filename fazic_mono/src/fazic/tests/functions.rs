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
