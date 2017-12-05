use ::fazic::*;

#[test]
fn add() {
    let mut fazic = Fazic::new();
    fazic.text_buffer.insert_string("? 1+1".to_string());
    fazic.enter_key();
    for _ in 0..100 { fazic.tick(); }
    fazic.text_buffer.cursor_line = fazic.text_buffer.cursor_line - 2;

    assert_eq!(
        fazic.text_buffer.get_current_line_string(),
        "2".to_string()
    )
}

#[test]
fn eq_true() {
    let mut fazic = Fazic::new();
    fazic.text_buffer.insert_string("? 1==1".to_string());
    fazic.enter_key();
    for _ in 0..100 { fazic.tick(); }
    fazic.text_buffer.cursor_line = fazic.text_buffer.cursor_line - 2;

    assert_eq!(
        fazic.text_buffer.get_current_line_string(),
        "true".to_string()
    )
}

#[test]
fn eq_false() {
    let mut fazic = Fazic::new();
    fazic.text_buffer.insert_string("? 1==2".to_string());
    fazic.enter_key();
    for _ in 0..100 { fazic.tick(); }
    fazic.text_buffer.cursor_line = fazic.text_buffer.cursor_line - 2;

    assert_eq!(
        fazic.text_buffer.get_current_line_string(),
        "false".to_string()
    )
}
