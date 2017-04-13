pub fn redraw_buffer(runtime : &mut ::fazic::Fazic) {
    runtime.screen.set_current_color(runtime.text.background_color);
    runtime.screen.clear();

    for i in 0..::fazic::TEXT_BUFFER_CHARS {
        let is_cursor = runtime.text.cursor == i && runtime.text.show_cursor;

        let color = if is_cursor {
            runtime.text.current_color
        } else {
            runtime.text.colors[i as usize]
        };

        runtime.screen.put_char(
            runtime.text.chars[i as usize],
            (i % ::fazic::TEXT_BUFFER_CHARS_PER_LINE * 8),
            (i / ::fazic::TEXT_BUFFER_CHARS_PER_LINE * 8),
            color,
            is_cursor,
            );
    }
}
