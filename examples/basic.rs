use std::ptr;

fn main() -> Result<(), ghostty_vt_sys::GhosttyResult> {
    let mut terminal = ptr::null_mut();
    // SAFETY: Null selects the default allocator; the output pointer is valid
    // and both terminal dimensions are nonzero.
    let result =
        unsafe { ghostty_vt_sys::ghostty_terminal_new(ptr::null(), &mut terminal, 80, 24) };
    if result != ghostty_vt_sys::GHOSTTY_SUCCESS {
        return Err(result);
    }
    assert!(
        !terminal.is_null(),
        "successful creation must return a handle"
    );

    let text = b"\x1b[31mHello\x1b[0m";
    let mut column = 0_u16;
    // SAFETY: The terminal is live, the input bytes remain valid during the
    // write, and CURSOR_X writes one uint16_t. Free the owned handle exactly
    // once, before returning or inspecting the query result.
    let result = unsafe {
        ghostty_vt_sys::ghostty_terminal_vt_write(terminal, text.as_ptr(), text.len());
        let result = ghostty_vt_sys::ghostty_terminal_get(
            terminal,
            ghostty_vt_sys::GHOSTTY_TERMINAL_DATA_CURSOR_X,
            ptr::from_mut(&mut column).cast(),
        );
        ghostty_vt_sys::ghostty_terminal_free(terminal);
        result
    };
    if result != ghostty_vt_sys::GHOSTTY_SUCCESS {
        return Err(result);
    }

    // The escape sequences change the style without occupying terminal cells.
    assert_eq!(column, 5);
    println!("Cursor column after writing styled text: {column}");
    Ok(())
}
