use fazic::*;

#[test]
fn add() {
    let mut fazic = Fazic::new();
    fazic.text_buffer.insert_string("? 1+1".to_string());
    fazic.enter_key();
    for _ in 0..100 {
        fazic.tick();
    }
    fazic.text_buffer.cursor_line = fazic.text_buffer.cursor_line - 2;

    assert_eq!(fazic.text_buffer.get_current_line_string(), "2".to_string())
}

#[test]
fn eq_true() {
    let mut fazic = Fazic::new();
    fazic.text_buffer.insert_string("? 1==1".to_string());
    fazic.enter_key();
    for _ in 0..100 {
        fazic.tick();
    }
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
    for _ in 0..100 {
        fazic.tick();
    }
    fazic.text_buffer.cursor_line = fazic.text_buffer.cursor_line - 2;

    assert_eq!(
        fazic.text_buffer.get_current_line_string(),
        "false".to_string()
    )
}

#[test]
fn gt_true() {
    let mut fazic = Fazic::new();
    fazic.text_buffer.insert_string("? 2>1".to_string());
    fazic.enter_key();
    for _ in 0..100 {
        fazic.tick();
    }
    fazic.text_buffer.cursor_line = fazic.text_buffer.cursor_line - 2;

    assert_eq!(
        fazic.text_buffer.get_current_line_string(),
        "true".to_string()
    )
}

#[test]
fn gt_false() {
    let mut fazic = Fazic::new();
    fazic.text_buffer.insert_string("? 1>2".to_string());
    fazic.enter_key();
    for _ in 0..100 {
        fazic.tick();
    }
    fazic.text_buffer.cursor_line = fazic.text_buffer.cursor_line - 2;

    assert_eq!(
        fazic.text_buffer.get_current_line_string(),
        "false".to_string()
    )
}

#[test]
fn lt_true() {
    let mut fazic = Fazic::new();
    fazic.text_buffer.insert_string("? 1<2".to_string());
    fazic.enter_key();
    for _ in 0..100 {
        fazic.tick();
    }
    fazic.text_buffer.cursor_line = fazic.text_buffer.cursor_line - 2;

    assert_eq!(
        fazic.text_buffer.get_current_line_string(),
        "true".to_string()
    )
}

#[test]
fn lt_false() {
    let mut fazic = Fazic::new();
    fazic.text_buffer.insert_string("? 2<1".to_string());
    fazic.enter_key();
    for _ in 0..100 {
        fazic.tick();
    }
    fazic.text_buffer.cursor_line = fazic.text_buffer.cursor_line - 2;

    assert_eq!(
        fazic.text_buffer.get_current_line_string(),
        "false".to_string()
    )
}

#[test]
fn lteq_true() {
    let mut fazic = Fazic::new();
    fazic.text_buffer.insert_string("? 1<=2".to_string());
    fazic.enter_key();
    for _ in 0..100 {
        fazic.tick();
    }
    fazic.text_buffer.cursor_line = fazic.text_buffer.cursor_line - 2;

    assert_eq!(
        fazic.text_buffer.get_current_line_string(),
        "true".to_string()
    )
}

#[test]
fn lteq_false() {
    let mut fazic = Fazic::new();
    fazic.text_buffer.insert_string("? 2<=1".to_string());
    fazic.enter_key();
    for _ in 0..100 {
        fazic.tick();
    }
    fazic.text_buffer.cursor_line = fazic.text_buffer.cursor_line - 2;

    assert_eq!(
        fazic.text_buffer.get_current_line_string(),
        "false".to_string()
    )
}

#[test]
fn neg() {
    let mut fazic = Fazic::new();
    fazic.text_buffer.insert_string("? -1".to_string());
    fazic.enter_key();
    for _ in 0..100 {
        fazic.tick();
    }
    fazic.text_buffer.cursor_line = fazic.text_buffer.cursor_line - 2;

    assert_eq!(
        fazic.text_buffer.get_current_line_string(),
        "-1".to_string()
    )
}

#[test]
fn and_bool() {
    let mut fazic = Fazic::new();
    fazic.text_buffer.insert_string("? 1<0 AND 1>1".to_string());
    fazic.enter_key();
    for _ in 0..100 {
        fazic.tick();
    }
    fazic.text_buffer.cursor_line = fazic.text_buffer.cursor_line - 2;

    assert_eq!(
        fazic.text_buffer.get_current_line_string(),
        "false".to_string()
    )
}

#[test]
fn and_bit() {
    let mut fazic = Fazic::new();
    fazic.text_buffer.insert_string("? 123 AND 23".to_string());
    fazic.enter_key();
    for _ in 0..100 {
        fazic.tick();
    }
    fazic.text_buffer.cursor_line = fazic.text_buffer.cursor_line - 2;

    assert_eq!(
        fazic.text_buffer.get_current_line_string(),
        "19".to_string()
    )
}

#[test]
fn or_bool() {
    let mut fazic = Fazic::new();
    fazic.text_buffer.insert_string("? 1<0 OR 1>1".to_string());
    fazic.enter_key();
    for _ in 0..100 {
        fazic.tick();
    }
    fazic.text_buffer.cursor_line = fazic.text_buffer.cursor_line - 2;

    assert_eq!(
        fazic.text_buffer.get_current_line_string(),
        "false".to_string()
    )
}

#[test]
fn or_bit() {
    let mut fazic = Fazic::new();
    fazic.text_buffer.insert_string("? 123 OR 23".to_string());
    fazic.enter_key();
    for _ in 0..100 {
        fazic.tick();
    }
    fazic.text_buffer.cursor_line = fazic.text_buffer.cursor_line - 2;

    assert_eq!(
        fazic.text_buffer.get_current_line_string(),
        "127".to_string()
    )
}

#[test]
fn not() {
    let mut fazic = Fazic::new();
    fazic.text_buffer.insert_string("? NOT(0)".to_string());
    fazic.enter_key();
    for _ in 0..100 {
        fazic.tick();
    }
    fazic.text_buffer.cursor_line = fazic.text_buffer.cursor_line - 2;

    assert_eq!(
        fazic.text_buffer.get_current_line_string(),
        "-1".to_string()
    )
}
