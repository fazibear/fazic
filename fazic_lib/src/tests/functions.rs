use *;

#[test]
fn abs() {
    let mut fazic = Fazic::new();
    fazic.text_buffer.insert_string("? ABS(-2)".to_string());
    fazic.enter_key();
    for _ in 0..100 {
        fazic.tick();
    }
    fazic.text_buffer.cursor_line = fazic.text_buffer.cursor_line - 2;

    assert_eq!(fazic.text_buffer.get_current_line_string(), "2".to_string())
}

#[test]
fn sin() {
    let mut fazic = Fazic::new();
    fazic.text_buffer.insert_string("? SIN(1)".to_string());
    fazic.enter_key();
    for _ in 0..100 {
        fazic.tick();
    }
    fazic.text_buffer.cursor_line = fazic.text_buffer.cursor_line - 2;

    assert_eq!(
        fazic.text_buffer.get_current_line_string(),
        "0.8414709848078965".to_string()
    )
}

#[test]
fn cos() {
    let mut fazic = Fazic::new();
    fazic.text_buffer.insert_string("? COS(1)".to_string());
    fazic.enter_key();
    for _ in 0..100 {
        fazic.tick();
    }
    fazic.text_buffer.cursor_line = fazic.text_buffer.cursor_line - 2;

    assert_eq!(
        fazic.text_buffer.get_current_line_string(),
        "0.5403023058681398".to_string()
    )
}

#[test]
fn tan() {
    let mut fazic = Fazic::new();
    fazic.text_buffer.insert_string("? TAN(2)".to_string());
    fazic.enter_key();
    for _ in 0..100 {
        fazic.tick();
    }
    fazic.text_buffer.cursor_line = fazic.text_buffer.cursor_line - 2;

    assert_eq!(
        fazic.text_buffer.get_current_line_string(),
        "-2.185039863261519".to_string()
    )
}

#[test]
fn atn() {
    let mut fazic = Fazic::new();
    fazic.text_buffer.insert_string("? ATN(1)".to_string());
    fazic.enter_key();
    for _ in 0..100 {
        fazic.tick();
    }
    fazic.text_buffer.cursor_line = fazic.text_buffer.cursor_line - 2;

    assert_eq!(
        fazic.text_buffer.get_current_line_string(),
        "0.7853981633974483".to_string()
    )
}

#[test]
fn log() {
    let mut fazic = Fazic::new();
    fazic.text_buffer.insert_string("? LOG(2)".to_string());
    fazic.enter_key();
    for _ in 0..100 {
        fazic.tick();
    }
    fazic.text_buffer.cursor_line = fazic.text_buffer.cursor_line - 2;

    assert_eq!(
        fazic.text_buffer.get_current_line_string(),
        "0.6931471805599453".to_string()
    )
}

#[test]
fn exp() {
    let mut fazic = Fazic::new();
    fazic.text_buffer.insert_string("? EXP(2)".to_string());
    fazic.enter_key();
    for _ in 0..100 {
        fazic.tick();
    }
    fazic.text_buffer.cursor_line = fazic.text_buffer.cursor_line - 2;

    assert_eq!(
        fazic.text_buffer.get_current_line_string(),
        "7.38905609893065".to_string()
    )
}

#[test]
fn sqr() {
    let mut fazic = Fazic::new();
    fazic.text_buffer.insert_string("? SQR(2)".to_string());
    fazic.enter_key();
    for _ in 0..100 {
        fazic.tick();
    }
    fazic.text_buffer.cursor_line = fazic.text_buffer.cursor_line - 2;

    assert_eq!(
        fazic.text_buffer.get_current_line_string(),
        "1.4142135623730951".to_string()
    )
}

#[test]
fn sgn() {
    let mut fazic = Fazic::new();
    fazic.text_buffer.insert_string("? SGN(2)".to_string());
    fazic.enter_key();
    for _ in 0..100 {
        fazic.tick();
    }
    fazic.text_buffer.cursor_line = fazic.text_buffer.cursor_line - 2;

    assert_eq!(fazic.text_buffer.get_current_line_string(), "1".to_string())
}

#[test]
fn len() {
    let mut fazic = Fazic::new();
    fazic
        .text_buffer
        .insert_string("? LEN(\"TEST 123\")".to_string());
    fazic.enter_key();
    for _ in 0..100 {
        fazic.tick();
    }
    fazic.text_buffer.cursor_line = fazic.text_buffer.cursor_line - 2;

    assert_eq!(fazic.text_buffer.get_current_line_string(), "8".to_string())
}

#[test]
fn chr() {
    let mut fazic = Fazic::new();
    fazic.text_buffer.insert_string("? CHR(44)".to_string());
    fazic.enter_key();
    for _ in 0..100 {
        fazic.tick();
    }
    fazic.text_buffer.cursor_line = fazic.text_buffer.cursor_line - 2;

    assert_eq!(fazic.text_buffer.get_current_line_string(), ",".to_string())
}

#[test]
fn asc() {
    let mut fazic = Fazic::new();
    fazic.text_buffer.insert_string("? ASC(\",\")".to_string());
    fazic.enter_key();
    for _ in 0..100 {
        fazic.tick();
    }
    fazic.text_buffer.cursor_line = fazic.text_buffer.cursor_line - 2;

    assert_eq!(
        fazic.text_buffer.get_current_line_string(),
        "44".to_string()
    )
}

#[test]
fn val() {
    let mut fazic = Fazic::new();
    fazic
        .text_buffer
        .insert_string("? VAL(\"123.12\")".to_string());
    fazic.enter_key();
    for _ in 0..100 {
        fazic.tick();
    }
    fazic.text_buffer.cursor_line = fazic.text_buffer.cursor_line - 2;

    assert_eq!(
        fazic.text_buffer.get_current_line_string(),
        "123.12".to_string()
    )
}

#[test]
fn int() {
    let mut fazic = Fazic::new();
    fazic.text_buffer.insert_string("? INT(123.12)".to_string());
    fazic.enter_key();
    for _ in 0..100 {
        fazic.tick();
    }
    fazic.text_buffer.cursor_line = fazic.text_buffer.cursor_line - 2;

    assert_eq!(
        fazic.text_buffer.get_current_line_string(),
        "123".to_string()
    )
}

#[test]
fn str() {
    let mut fazic = Fazic::new();
    fazic.text_buffer.insert_string("? STR(123.12)".to_string());
    fazic.enter_key();
    for _ in 0..100 {
        fazic.tick();
    }
    fazic.text_buffer.cursor_line = fazic.text_buffer.cursor_line - 2;

    assert_eq!(
        fazic.text_buffer.get_current_line_string(),
        "123.12".to_string()
    )
}
